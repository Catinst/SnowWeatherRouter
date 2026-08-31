#!/usr/bin/env python3
"""Build a Snow-routed clone APK from an uploaded Xiaomi Weather APK."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

SIGNATURE_RE = re.compile(
    r"^META-INF/(?:MANIFEST\.MF|[^/]+\.(?:SF|RSA|DSA|EC))$", re.IGNORECASE
)
PACKAGE_RE = re.compile(
    r"package: name='([^']+)' versionCode='([^']+)' versionName='([^']*)'"
)


def run(command: list[str], capture: bool = False) -> str:
    print("+", subprocess.list2cmdline(command))
    result = subprocess.run(
        command,
        check=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=capture,
    )
    return result.stdout if capture else ""


def find_executable(name: str, explicit: str | None = None) -> str:
    if explicit:
        return explicit
    found = shutil.which(name)
    if found:
        return found
    raise SystemExit(f"{name} was not found; pass an explicit path")


def find_android_jar(sdk: Path, explicit: str | None = None) -> Path:
    if explicit:
        path = Path(explicit)
        if path.is_file():
            return path
        raise SystemExit(f"android.jar not found: {path}")
    candidates = sorted(
        sdk.glob("platforms/android-*/android.jar"),
        key=lambda path: path.parent.name,
        reverse=True,
    )
    if not candidates:
        raise SystemExit(f"No android.jar found under {sdk / 'platforms'}")
    return candidates[0]


def find_aapt2(sdk: Path, explicit: str | None = None) -> str:
    if explicit:
        return explicit
    name = "aapt2.exe" if os.name == "nt" else "aapt2"
    candidates = sorted(
        sdk.glob(f"build-tools/*/{name}"),
        key=lambda path: path.parent.name,
        reverse=True,
    )
    if not candidates:
        return find_executable("aapt2")
    return str(candidates[0])


def apk_metadata(aapt2: str, apk: Path) -> tuple[str, int, str]:
    output = run([aapt2, "dump", "badging", str(apk)], capture=True)
    match = PACKAGE_RE.search(output)
    if not match:
        raise SystemExit("Unable to parse package metadata from aapt2 dump badging")
    return match.group(1), int(match.group(2)), match.group(3)


def manifest_xml(
    package: str,
    version_code: int,
    version_name: str,
    label: str,
) -> str:
    return f'''<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{package}"
    android:versionCode="{version_code}"
    android:versionName="{version_name}">
    <uses-sdk android:minSdkVersion="30" android:targetSdkVersion="36" />
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.VIBRATE" />
    <application
        android:label="{label}"
        android:name="android.app.Application"
        android:hasCode="false"
        android:hardwareAccelerated="true"
        android:extractNativeLibs="true"
        android:supportsRtl="true">
        <activity
            android:name="android.app.NativeActivity"
            android:exported="true"
            android:launchMode="singleTask"
            android:screenOrientation="portrait"
            android:hardwareAccelerated="true"
            android:configChanges="orientation|keyboardHidden|keyboard|screenSize|smallestScreenSize|screenLayout|uiMode|density|fontScale|locale|layoutDirection">
            <meta-data android:name="android.app.lib_name" android:value="SnowWeatherRouter" />
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
'''


def compile_manifest(
    aapt2: str,
    android_jar: Path,
    xml: str,
    work: Path,
) -> bytes:
    xml_path = work / "AndroidManifest.xml"
    template_apk = work / "manifest-template.apk"
    xml_path.write_text(xml, encoding="utf-8")
    run(
        [
            aapt2,
            "link",
            "--manifest",
            str(xml_path),
            "-I",
            str(android_jar),
            "-o",
            str(template_apk),
        ]
    )
    with zipfile.ZipFile(template_apk) as archive:
        return archive.read("AndroidManifest.xml")


def clone_info(info: zipfile.ZipInfo) -> zipfile.ZipInfo:
    output = zipfile.ZipInfo(info.filename, info.date_time)
    output.compress_type = info.compress_type
    output.comment = info.comment
    output.extra = info.extra
    output.create_system = info.create_system
    output.create_version = info.create_version
    output.extract_version = info.extract_version
    output.reserved = info.reserved
    output.volume = info.volume
    output.internal_attr = info.internal_attr
    output.external_attr = info.external_attr
    return output


def rewrite_apk(
    source: Path,
    output: Path,
    manifest: bytes,
    router: bytes,
    source_package: str,
    target_package: str,
) -> dict[str, object]:
    if len(source_package) != len(target_package):
        raise SystemExit(
            "Source and target package names must have equal byte length for safe AOT/resource replacement"
        )
    old_ascii = source_package.encode("ascii")
    new_ascii = target_package.encode("ascii")
    old_utf16 = source_package.encode("utf-16le")
    new_utf16 = target_package.encode("utf-16le")

    removed: list[str] = []
    changed: dict[str, dict[str, int]] = {}
    seen_router = False
    source_names: list[str] = []
    with zipfile.ZipFile(source, "r") as src, zipfile.ZipFile(
        output, "w", allowZip64=True
    ) as dst:
        for info in src.infolist():
            source_names.append(info.filename)
            if SIGNATURE_RE.match(info.filename):
                removed.append(info.filename)
                continue
            if info.filename == "AndroidManifest.xml":
                copied = clone_info(info)
                dst.writestr(
                    copied,
                    manifest,
                    compress_type=zipfile.ZIP_DEFLATED,
                    compresslevel=9,
                )
                changed[info.filename] = {"manifest": 1}
                continue
            if info.filename == "lib/arm64-v8a/libSnowWeatherRouter.so":
                copied = clone_info(info)
                dst.writestr(copied, router, compress_type=zipfile.ZIP_STORED)
                seen_router = True
                changed[info.filename] = {"router": 1}
                continue

            data = src.read(info)
            ascii_hits = data.count(old_ascii) if source_package != target_package else 0
            utf16_hits = data.count(old_utf16) if source_package != target_package else 0
            if ascii_hits or utf16_hits:
                data = data.replace(old_ascii, new_ascii).replace(old_utf16, new_utf16)
                changed[info.filename] = {
                    "ascii_package_replacements": ascii_hits,
                    "utf16_package_replacements": utf16_hits,
                }
            copied = clone_info(info)
            if info.compress_type == zipfile.ZIP_DEFLATED:
                dst.writestr(
                    copied,
                    data,
                    compress_type=info.compress_type,
                    compresslevel=9,
                )
            else:
                dst.writestr(copied, data, compress_type=info.compress_type)

        if not seen_router:
            info = zipfile.ZipInfo("lib/arm64-v8a/libSnowWeatherRouter.so")
            info.compress_type = zipfile.ZIP_STORED
            info.create_system = 3
            info.external_attr = 0o100755 << 16
            dst.writestr(info, router, compress_type=zipfile.ZIP_STORED)
            changed[info.filename] = {"router": 1}

    with zipfile.ZipFile(output, "r") as result:
        bad = result.testzip()
        if bad is not None:
            raise SystemExit(f"Output APK CRC failure: {bad}")
        if result.read("lib/arm64-v8a/libSnowWeatherRouter.so") != router:
            raise SystemExit("Embedded Snow router verification failed")
        if result.read("AndroidManifest.xml") != manifest:
            raise SystemExit("Embedded manifest verification failed")

    return {
        "removed_signatures": removed,
        "changed_entries": changed,
        "output_size": output.stat().st_size,
        "output_sha256": hashlib.sha256(output.read_bytes()).hexdigest(),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input_apk", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--target-package", default="com.miui.weather3")
    parser.add_argument("--version-code-increment", type=int, default=1)
    parser.add_argument("--version-name-suffix", default="-Snow")
    parser.add_argument("--label", default="Snow Weather")
    parser.add_argument("--sdk", type=Path, default=Path(os.environ.get("ANDROID_SDK_ROOT", os.environ.get("ANDROID_HOME", ""))))
    parser.add_argument("--aapt2")
    parser.add_argument("--android-jar")
    parser.add_argument("--rustc")
    args = parser.parse_args()

    repo = Path(__file__).resolve().parents[1]
    sdk = args.sdk
    if not str(sdk):
        raise SystemExit("Pass --sdk or set ANDROID_SDK_ROOT")
    aapt2 = find_aapt2(sdk, args.aapt2)
    android_jar = find_android_jar(sdk, args.android_jar)
    source_package, source_version_code, source_version_name = apk_metadata(
        aapt2, args.input_apk.resolve()
    )
    output = (
        args.output.resolve()
        if args.output
        else args.input_apk.resolve().with_name(
            f"{args.input_apk.stem}-Snow-unsigned.apk"
        )
    )

    with tempfile.TemporaryDirectory(prefix="snow-weather-") as temp_name:
        work = Path(temp_name)
        router_dir = work / "router"
        run(
            [
                sys.executable,
                str(repo / "scripts" / "build_router.py"),
                str(args.input_apk.resolve()),
                "--output-dir",
                str(router_dir),
                *(["--rustc", args.rustc] if args.rustc else []),
            ]
        )
        router = (router_dir / "libSnowWeatherRouter.so").read_bytes()
        version_name = source_version_name + args.version_name_suffix
        manifest = compile_manifest(
            aapt2,
            android_jar,
            manifest_xml(
                args.target_package,
                source_version_code + args.version_code_increment,
                version_name,
                args.label,
            ),
            work,
        )
        result = rewrite_apk(
            args.input_apk.resolve(),
            output,
            manifest,
            router,
            source_package,
            args.target_package,
        )

    result.update(
        {
            "input": str(args.input_apk.resolve()),
            "output": str(output),
            "source_package": source_package,
            "target_package": args.target_package,
            "source_version_code": source_version_code,
            "target_version_code": source_version_code + args.version_code_increment,
            "source_version_name": source_version_name,
            "target_version_name": source_version_name + args.version_name_suffix,
            "router_size": len(router),
            "router_sha256": hashlib.sha256(router).hexdigest(),
            "android_jar": str(android_jar),
            "aapt2": aapt2,
        }
    )
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
