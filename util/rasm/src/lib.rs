use std::{
    fs::{self, create_dir_all, remove_dir_all},
    io::{self, ErrorKind::NotFound, IsTerminal, Write},
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;

// TODO: custom build dir

// TODO: consider stderr where errors occur

// TODO: metadata system in build dir

// TODO: add unit testing

// TODO: add integration testing

macro_rules! proc_cmd {
    ($name:expr $(, $arg:expr)* $(,)?) => {
        std::process::Command::new($name) $(.arg($arg))*
    };
}

const ABOUT: &str = "\
Compiles, Links, and Runs asm.
if <files> are ommited, reads all files in base.";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<ColorMode> for termcolor::ColorChoice {
    fn from(value: ColorMode) -> Self {
        use termcolor::ColorChoice::*;
        match value {
            ColorMode::Auto => Auto,
            ColorMode::Always => Always,
            ColorMode::Never => Never,
        }
    }
}

#[derive(Default, Deserialize, Parser)]
#[command(name = "rasm")]
#[command(version = "0.0.1")]
#[command(about = ABOUT, long_about = None)]
#[command(styles=get_styles())]
struct Config {
    /// Name of file to run, relative to base
    #[serde(default)]
    #[arg(short, long)]
    run: Option<PathBuf>,
    /// No stdout
    #[serde(default)]
    #[arg(short, long)]
    quiet: bool,
    /// Clean build dir after compiling & running
    #[serde(default)]
    #[arg(long)]
    clean: bool,
    /// Color Mode, auto, never, always
    #[serde(default)]
    #[arg(short, long)]
    color: Option<ColorMode>,
    /// Base path to look for files in
    ///
    /// Defaults to src
    #[serde(default)]
    #[arg(short, long)]
    base: Option<PathBuf>,
    /// Files to compile, relative to base
    #[serde(default)]
    files: Option<Vec<PathBuf>>,
}

fn read_files(files: Option<Vec<PathBuf>>, base: &Path) -> Vec<PathBuf> {
    fn filter(p: &Path) -> bool {
        p.exists()
            && p.is_file()
            && !p.is_symlink()
            && p.extension()
                .and_then(|os| os.to_str())
                .is_some_and(|s| matches!(s, "s" | "S" | "asm"))
    }
    let read_files = |files: Vec<_>| {
        files
            .into_iter()
            .map(|p| base.join(p))
            .filter(|p| filter(p))
            .collect()
    };
    let read_all = || {
        WalkDir::new(base)
            .into_iter()
            .flatten()
            .filter_map(|p| filter(p.path()).then(|| p.into_path()))
            .collect()
    };
    if !base.exists() {
        return Vec::new();
    }
    let mut files: Vec<_> = files.map(read_files).unwrap_or_else(read_all);
    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    files
}

fn check_tooling() -> anyhow::Result<process::Output> {
    match proc_cmd!("nasm", "--version").output() {
        Ok(c) => Ok(c),
        Err(e) if e.kind() == NotFound => Err(anyhow::anyhow!("nasm not found")),
        Err(e) => Err(e.into()),
    }
}

fn read_config() -> Config {
    let path = Path::new(".rasm.toml");
    if !path.exists() {
        return Config::default();
    }
    let Ok(cfg_src) = fs::read_to_string(path) else {
        return Config::default();
    };
    toml::from_str(&cfg_src).unwrap_or_else(|_| Config::default())
}

fn init() -> Config {
    fn merge<T>(a: Option<T>, b: Option<T>) -> Option<T> {
        if let Some(a) = a {
            return Some(a);
        }
        b
    }
    let args = Config::parse();
    let config = read_config();
    Config {
        run: merge(args.run, config.run),
        quiet: if args.quiet { true } else { config.quiet },
        clean: if args.clean { true } else { config.clean },
        color: merge(args.color, config.color),
        base: merge(args.base, config.base),
        files: merge(args.files, config.files),
    }
}

pub fn run() -> anyhow::Result<()> {
    let args = init();
    let mut writer = ColorWriter::new(args.color.unwrap_or(ColorMode::Auto), args.quiet);

    if args.quiet {
        std::env::set_var("QUIET", "true");
    } else {
        std::env::remove_var("QUIET");
    }

    check_tooling()?;

    let src_base = args.base.unwrap_or(PathBuf::from("src"));
    let obj_base = PathBuf::from("build/obj");
    let bin_base = PathBuf::from("build/bin");

    let files = read_files(args.files, &src_base);

    if files.is_empty() {
        writer.fg(Some(Color::Red))?;
        writeln!(
            writer,
            "No assembly found in path: {}",
            src_base.to_string_lossy()
        )?;
        writer.reset()?;
        return Ok(());
    }

    let paths: Vec<_> = files
        .iter()
        .flat_map(|src| {
            let asrc = src.strip_prefix(&src_base).ok()?;
            let obj = obj_base.join(asrc).with_extension("o");
            let bin = bin_base.join(asrc).with_extension("");
            Some((src, asrc, obj, bin))
        })
        .collect();

    create_dir_all(&src_base)?;
    create_dir_all(&obj_base)?;
    create_dir_all(&bin_base)?;

    writer.fg(Some(Color::Blue))?;
    writeln!(
        writer,
        "{} files found, {} paths valid\nCompiling",
        files.len(),
        paths.len()
    )?;
    writer.reset()?;

    paths.iter().try_for_each(|(src, asrc, obj, _)| {
        writeln!(writer, "- {}", asrc.to_string_lossy())?;
        proc_cmd!("nasm", "-f", "elf64", &src, "-o", &obj)
            .spawn()?
            .wait_with_output()?;
        io::Result::Ok(())
    })?;

    writer.fg(Some(Color::Blue))?;
    writeln!(writer, "\nLinking")?;
    writer.reset()?;

    paths.iter().try_for_each(|(_, asrc, obj, bin)| {
        writeln!(writer, "- {}", asrc.to_string_lossy())?;
        proc_cmd!("ld", &obj, "-o", &bin)
            .spawn()?
            .wait_with_output()?;
        io::Result::Ok(())
    })?;

    writer.fg(Some(Color::Green))?;
    writeln!(writer, "\nDone")?;
    writer.reset()?;

    if let Some(ref file) = args.run {
        let file = bin_base.join(file);
        if file.exists() {
            writer.fg(Some(Color::Blue))?;
            writeln!(writer, "running {}:\n", file.to_string_lossy())?;
            writer.reset()?;
            proc_cmd!(file).spawn()?.wait_with_output()?;
        } else {
            writer.fg(Some(Color::Red))?;
            writeln!(writer, "\nfile not found: {}:\n", file.to_string_lossy())?;
            writer.reset()?;
        }
    }

    if args.clean {
        writer.fg(Some(Color::Blue))?;
        writeln!(writer, "\ncleaning files...")?;
        writer.reset()?;
        remove_dir_all(obj_base)?;
        remove_dir_all(bin_base)?;
    }
    if args.clean || args.run.is_some() {
        writer.fg(Some(Color::Green))?;
        writeln!(writer, "\nDone")?;
        writer.reset()?;
    }

    Ok(())
}

struct ColorWriter {
    q: bool,
    w: StandardStream,
}

#[allow(unused)]
impl ColorWriter {
    fn new(mut color: ColorMode, quiet: bool) -> Self {
        if color == ColorMode::Auto && !std::io::stdin().is_terminal() {
            color = ColorMode::Never;
        }
        let w = StandardStream::stdout(color.into());
        let q = quiet;
        Self { q, w }
    }
    fn reset(&mut self) -> io::Result<()> {
        self.w.set_color(ColorSpec::new().set_reset(true))?;
        self.w.flush()
    }
    fn fg(&mut self, color: Option<Color>) -> io::Result<()> {
        self.w.set_color(ColorSpec::new().set_fg(color))
    }
    fn bg(&mut self, color: Option<Color>) -> io::Result<()> {
        self.w.set_color(ColorSpec::new().set_bg(color))
    }
}

impl io::Write for ColorWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.q {
            return Ok(0);
        };
        self.w.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        if self.q {
            return Ok(0);
        };
        self.w.write_vectored(bufs)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        if self.q {
            return Ok(());
        };
        self.w.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        if self.q {
            return Ok(());
        };
        self.w.write_fmt(fmt)
    }
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
