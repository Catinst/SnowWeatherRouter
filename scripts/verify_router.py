#!/usr/bin/env python3
"""Verify only that the Snow router watermark name is present."""
from __future__ import annotations

import argparse
from pathlib import Path

WATERMARK_NAME = b"SnowWeatherRouter"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("library", type=Path)
    args = parser.parse_args()
    payload = args.library.read_bytes()
    if WATERMARK_NAME not in payload:
        raise SystemExit("SnowWeatherRouter watermark name is missing")
    print(f"path={args.library}")
    print(f"size={len(payload)}")
    print("watermark=SnowWeatherRouter")


if __name__ == "__main__":
    main()
