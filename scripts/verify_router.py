#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
from elftools.elf.elffile import ELFFile

REQUIRED_NEEDED = {
    "libhyper_os_flutter.so",
    "libapp.so",
    "libandroid.so",
    "liblog.so",
}
REQUIRED_SYMBOLS = {
    "ANativeActivity_onCreate",
    "SNOW_WEATHER_ROUTER_WATERMARK",
    "SNOW_WEATHER_ROUTER_BUILD",
    "Java_com_miui_weather3_ActivityWeatherMain_nativeSetUserAgreement",
    "Java_com_miui_weather3_ActivityWeatherMain_nativeSetLocationPermission",
    "Java_com_miui_weather3_ActivityWeatherMain_nativeDeliverActivityResult",
    "Java_com_miui_weather3_ActivityWeatherMain_nativeDeliverPermissionResult",
}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("library", type=Path)
    args = parser.parse_args()
    payload = args.library.read_bytes()
    if b"SnowWeatherRouter|Snownight|v18-hybrid\0" not in payload:
        raise SystemExit("Snow watermark is missing")

    with args.library.open("rb") as stream:
        elf = ELFFile(stream)
        if elf["e_machine"] != "EM_AARCH64" or elf["e_type"] != "ET_DYN":
            raise SystemExit(f"unexpected ELF: {elf['e_machine']} {elf['e_type']}")
        dynsym = elf.get_section_by_name(".dynsym")
        symbols = {symbol.name for symbol in dynsym.iter_symbols() if symbol.name}
        missing_symbols = REQUIRED_SYMBOLS - symbols
        if missing_symbols:
            raise SystemExit(f"missing exported symbols: {sorted(missing_symbols)}")
        dynamic = elf.get_section_by_name(".dynamic")
        needed: set[str] = set()
        soname = None
        for tag in dynamic.iter_tags():
            if tag.entry.d_tag == "DT_NEEDED":
                needed.add(tag.needed)
            elif tag.entry.d_tag == "DT_SONAME":
                soname = tag.soname
        if soname != "libSnowWeatherRouter.so":
            raise SystemExit(f"unexpected SONAME: {soname!r}")
        missing_needed = REQUIRED_NEEDED - needed
        if missing_needed:
            raise SystemExit(f"missing DT_NEEDED entries: {sorted(missing_needed)}")

    print(f"path={args.library}")
    print(f"size={len(payload)}")
    print(f"sha256={hashlib.sha256(payload).hexdigest()}")
    print("watermark=SnowWeatherRouter|Snownight|v18-hybrid")
    print(f"needed={','.join(sorted(needed))}")
    print(f"symbols={','.join(sorted(REQUIRED_SYMBOLS))}")


if __name__ == "__main__":
    main()
