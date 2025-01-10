import os
import glob
import subprocess
import pathlib
import argparse
import shutil

parser = argparse.ArgumentParser(
    prog='clrasm',
    description='Compiles, Links, and Runs asm using nasm',
    epilog='Made By 4lineclear',
)
parser.add_argument('-r', '--run', type=str, help="runs the given file")
parser.add_argument('-c', '--clean', action='store_true', help="cleans the build dir")
args = parser.parse_args()

if not os.path.exists("src"):
    os.makedirs("src")
if not os.path.exists("build"):
    os.makedirs("build")

for file in glob.glob("src/*.asm"):
    path = pathlib.Path(file)
    nasm_path = pathlib.Path(*path.with_suffix(".o").parts[1:])
    ld_path = pathlib.Path(*path.with_suffix("").parts[1:])
    subprocess.call(["nasm", "-f", "elf64", file, "-o", f"build/{nasm_path}"])
    subprocess.call(["ld", f"build/{nasm_path}", "-o", f"build/{ld_path}"])

if args.run is not None:
    subprocess.call([f"build/{args.run}"])
if args.clean:
    shutil.rmtree("build")
