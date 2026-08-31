#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
from pathlib import Path


def run(command: list[str]) -> None:
    print("+", subprocess.list2cmdline(command))
    subprocess.run(command, check=True)


def latest_android_jar(sdk: Path) -> Path:
    candidates = sorted(
        sdk.glob("platforms/android-*/android.jar"),
        key=lambda path: path.parent.name,
        reverse=True,
    )
    if not candidates:
        raise SystemExit(f"android.jar not found under {sdk / 'platforms'}")
    return candidates[0]


def latest_d8(sdk: Path) -> Path:
    name = "d8.bat" if os.name == "nt" else "d8"
    candidates = sorted(
        sdk.glob(f"build-tools/*/{name}"),
        key=lambda path: path.parent.name,
        reverse=True,
    )
    if not candidates:
        raise SystemExit(f"d8 not found under {sdk / 'build-tools'}")
    return candidates[0]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sdk", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, default=Path("dist/hybrid-dex"))
    parser.add_argument("--javac")
    parser.add_argument("--min-api", type=int, default=30)
    args = parser.parse_args()

    repo = Path(__file__).resolve().parents[1]
    source = repo / "hybrid" / "src" / "com" / "miui" / "weather3" / "ActivityWeatherMain.java"
    output = args.output_dir.resolve()
    classes = output / "classes"
    dex = output / "dex"
    if output.exists():
        shutil.rmtree(output)
    classes.mkdir(parents=True)
    dex.mkdir(parents=True)

    javac = args.javac or shutil.which("javac")
    if not javac:
        raise SystemExit("javac was not found; pass --javac")
    android_jar = latest_android_jar(args.sdk)
    d8 = latest_d8(args.sdk)

    run([
        javac,
        "--release",
        "17",
        "-cp",
        str(android_jar),
        "-d",
        str(classes),
        str(source),
    ])
    class_file = classes / "com" / "miui" / "weather3" / "ActivityWeatherMain.class"
    run([
        str(d8),
        "--lib",
        str(android_jar),
        "--min-api",
        str(args.min_api),
        "--output",
        str(dex),
        str(class_file),
    ])
    print(f"output={dex / 'classes.dex'}")


if __name__ == "__main__":
    main()
