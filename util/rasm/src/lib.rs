use std::fs::{self, create_dir_all, remove_dir_all};
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

use anyhow::ensure;
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;
use walkdir::WalkDir;

// TODO: create better clean system
macro_rules! println {
    ($($token:tt)*) => {
        if !std::env::var("QUIET").is_ok() {
            std::println!($($token)*)
        }
    };
}

const ABOUT: &str = "\
Compiles, Links, and Runs asm.
if <files> are ommited, reads all files in base.";

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
    #[arg(short, long)]
    clean: bool,
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

fn base_path() -> PathBuf {
    PathBuf::from("src")
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
    files.map(read_files).unwrap_or_else(read_all)
}

fn check_tooling() -> anyhow::Result<process::Output> {
    match Command::new("nasm").arg("--version").output() {
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
    let args = Config::parse();
    let mut config = read_config();
    if let Some(run) = args.run {
        config.run = Some(run);
    }
    if args.quiet {
        config.quiet = true;
    }
    if args.clean {
        config.clean = true;
    }
    if let Some(base) = args.base {
        config.base = Some(base);
    }
    if let Some(files) = args.files {
        config.files = Some(files);
    }
    config
}

pub fn run() -> anyhow::Result<()> {
    let args = init();
    if args.quiet {
        std::env::set_var("QUIET", "true");
    }
    let base = args.base.unwrap_or_else(base_path);
    ensure!(base.exists(), "src directory not found\n");

    check_tooling()?;

    let files = read_files(args.files, &base);
    let build_base = PathBuf::from("build");

    println!("Compiling & Linking Files:");

    create_dir_all("build")?;

    files.par_iter().try_for_each(|file_source| {
        let with_extension = |ext: &str| {
            let mut file = build_base.join(file_source.strip_prefix("src")?);
            file.set_extension(ext);
            anyhow::Ok(file)
        };
        let object_file = with_extension("o")?;
        let output_file = with_extension("")?;

        let src = file_source.to_string_lossy();
        let obj = object_file.to_string_lossy();
        let out = output_file.to_string_lossy();

        println!("- {} to {} to {}", &src, &obj, &out);
        Command::new("nasm")
            .args(["-f", "elf64", &src, "-o", &obj])
            .spawn()?
            .wait_with_output()?;
        Command::new("ld")
            .args([&obj, "-o", &out])
            .spawn()?
            .wait_with_output()?;
        anyhow::Ok(())
    })?;

    if let Some(file) = args.run {
        let file = build_base.join(&file);
        println!("running {}:\n", file.to_string_lossy());
        Command::new(file).spawn()?.wait_with_output()?;
    }

    if args.clean {
        println!("\ncleaning files...");
        remove_dir_all(build_base)?;
    }
    Ok(())
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
