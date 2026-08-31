#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path


def run(command: list[str], cwd: Path | None = None) -> None:
    print("+", subprocess.list2cmdline(command))
    subprocess.run(command, cwd=cwd, check=True)


def extract_inputs(apk: Path, directory: Path) -> None:
    directory.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(apk) as archive:
        for entry in (
            "lib/arm64-v8a/libhyper_os_flutter.so",
            "lib/arm64-v8a/libapp.so",
        ):
            output = directory / Path(entry).name
            output.write_bytes(archive.read(entry))
            print(f"extracted={entry}->{output}")


def rustc_path(value: str | None) -> str:
    if value:
        return value
    found = shutil.which("rustc")
    if found:
        return found
    raise SystemExit("rustc was not found; pass --rustc")


def build_stub(rustc: str, source: Path, output: Path, soname: str) -> None:
    run(
        [
            rustc,
            "--edition=2021",
            "--target",
            "aarch64-linux-android",
            "--crate-type",
            "cdylib",
            "-C",
            "panic=abort",
            "-C",
            "linker=rust-lld",
            "-C",
            "link-arg=--build-id=none",
            "-C",
            f"link-arg=-soname={soname}",
            "-O",
            str(source),
            "-o",
            str(output),
        ]
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input_apk", type=Path)
    parser.add_argument("--output-dir", type=Path, default=Path("dist"))
    parser.add_argument("--rustc")
    args = parser.parse_args()

    repo = Path(__file__).resolve().parents[1]
    router = repo / "router"
    tools = repo / ".tools"
    extracted = tools / "input-libs"
    stubs = tools / "link-stubs"
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=True)
    stubs.mkdir(parents=True, exist_ok=True)
    rustc = rustc_path(args.rustc)

    extract_inputs(args.input_apk.resolve(), extracted)
    build_stub(rustc, router / "link_stubs" / "libandroid.rs", stubs / "libandroid.so", "libandroid.so")
    build_stub(rustc, router / "link_stubs" / "liblog.rs", stubs / "liblog.so", "liblog.so")
    build_stub(rustc, router / "link_stubs" / "libc.rs", stubs / "libc.so", "libc.so")
    build_stub(rustc, router / "link_stubs" / "libdl.rs", stubs / "libdl.so", "libdl.so")
    shutil.copy2(extracted / "libhyper_os_flutter.so", stubs / "libhyper_os_flutter.so")
    shutil.copy2(extracted / "libapp.so", stubs / "libapp.so")

    output_so = output / "libSnowWeatherRouter.so"
    run(
        [
            rustc,
            "--edition=2021",
            "--target",
            "aarch64-linux-android",
            "--crate-type",
            "cdylib",
            "-C",
            "panic=abort",
            "-C",
            "opt-level=z",
            "-C",
            "lto=fat",
            "-C",
            "codegen-units=1",
            "-C",
            "strip=symbols",
            "-C",
            "linker=rust-lld",
            "-C",
            "link-arg=--build-id=sha1",
            "-C",
            "link-arg=-soname=libSnowWeatherRouter.so",
            "-C",
            "link-arg=--no-as-needed",
            "-C",
            f"link-arg=-L{stubs}",
            "-C",
            "link-arg=-l:libhyper_os_flutter.so",
            "-C",
            "link-arg=-l:libapp.so",
            "-C",
            "link-arg=-l:libandroid.so",
            "-C",
            "link-arg=-l:liblog.so",
            "-C",
            "link-arg=-l:libc.so",
            "-C",
            "link-arg=-l:libdl.so",
            "-C",
            "link-arg=--allow-shlib-undefined",
            str(router / "src" / "lib.rs"),
            "-o",
            str(output_so),
        ]
    )
    run([sys.executable, str(repo / "scripts" / "verify_router.py"), str(output_so)])
    print(f"output={output_so}")


if __name__ == "__main__":
    main()
