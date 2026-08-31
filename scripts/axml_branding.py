#!/usr/bin/env python3
"""Extract and restore application branding in Android binary XML manifests."""
from __future__ import annotations

import struct
from dataclasses import dataclass

RES_STRING_POOL_TYPE = 0x0001
RES_XML_RESOURCE_MAP_TYPE = 0x0180
RES_XML_START_ELEMENT_TYPE = 0x0102
RES_XML_END_ELEMENT_TYPE = 0x0103
TYPE_REFERENCE = 0x01
TYPE_STRING = 0x03
NO_INDEX = 0xFFFFFFFF

ANDROID_THEME = 0x01010000
ANDROID_LABEL = 0x01010001
ANDROID_ICON = 0x01010002
ANDROID_NAME = 0x01010003

PLACEHOLDER_LABEL = 0x0104000A  # @android:string/ok
PLACEHOLDER_ICON = 0x0108009B   # @android:drawable/ic_dialog_info
PLACEHOLDER_THEME = 0x01030241  # @android:style/Theme.Material.Light.NoActionBar


@dataclass(frozen=True)
class Attribute:
    resource_id: int
    raw_value: str | None
    data_type: int
    data: int
    data_offset: int


@dataclass(frozen=True)
class StartElement:
    name: str
    attributes: tuple[Attribute, ...]

    def attribute(self, resource_id: int) -> Attribute | None:
        return next(
            (attribute for attribute in self.attributes if attribute.resource_id == resource_id),
            None,
        )


@dataclass(frozen=True)
class EndElement:
    name: str


@dataclass(frozen=True)
class Branding:
    label_resource: int
    icon_resource: int
    theme_resource: int
    launcher_activity: str | None

    def as_dict(self) -> dict[str, object]:
        return {
            "label_resource": f"0x{self.label_resource:08x}",
            "icon_resource": f"0x{self.icon_resource:08x}",
            "theme_resource": f"0x{self.theme_resource:08x}",
            "launcher_activity": self.launcher_activity,
        }


def u16(data: bytes | bytearray, offset: int) -> int:
    return struct.unpack_from("<H", data, offset)[0]


def u32(data: bytes | bytearray, offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]


def p32(data: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<I", data, offset, value)


def read_length8(data: bytes, offset: int) -> tuple[int, int]:
    value = data[offset]
    offset += 1
    if value & 0x80:
        value = ((value & 0x7F) << 8) | data[offset]
        offset += 1
    return value, offset


def read_length16(data: bytes, offset: int) -> tuple[int, int]:
    value = u16(data, offset)
    offset += 2
    if value & 0x8000:
        value = ((value & 0x7FFF) << 16) | u16(data, offset)
        offset += 2
    return value, offset


def parse_string_pool(data: bytes, chunk_offset: int) -> list[str]:
    header_size = u16(data, chunk_offset + 2)
    string_count = u32(data, chunk_offset + 8)
    flags = u32(data, chunk_offset + 16)
    strings_start = u32(data, chunk_offset + 20)
    utf8 = bool(flags & 0x100)
    offsets = [
        u32(data, chunk_offset + header_size + index * 4)
        for index in range(string_count)
    ]
    strings: list[str] = []
    for relative in offsets:
        position = chunk_offset + strings_start + relative
        if utf8:
            _, position = read_length8(data, position)
            byte_length, position = read_length8(data, position)
            strings.append(data[position : position + byte_length].decode("utf-8", "replace"))
        else:
            char_length, position = read_length16(data, position)
            strings.append(
                data[position : position + char_length * 2].decode("utf-16le", "replace")
            )
    return strings


def parse_events(data: bytes) -> list[StartElement | EndElement]:
    if len(data) < 8 or u16(data, 0) != 0x0003:
        raise ValueError("not an Android binary XML document")
    position = 8
    strings: list[str] = []
    resource_map: list[int] = []
    events: list[StartElement | EndElement] = []
    while position < len(data):
        chunk_type = u16(data, position)
        header_size = u16(data, position + 2)
        chunk_size = u32(data, position + 4)
        if header_size < 8 or chunk_size < header_size or position + chunk_size > len(data):
            raise ValueError(f"invalid chunk at 0x{position:x}")
        if chunk_type == RES_STRING_POOL_TYPE:
            strings = parse_string_pool(data, position)
        elif chunk_type == RES_XML_RESOURCE_MAP_TYPE:
            count = (chunk_size - header_size) // 4
            resource_map = [
                u32(data, position + header_size + index * 4) for index in range(count)
            ]
        elif chunk_type == RES_XML_START_ELEMENT_TYPE:
            extension = position + header_size
            name_index = u32(data, extension + 4)
            attribute_start = u16(data, extension + 8)
            attribute_size = u16(data, extension + 10)
            attribute_count = u16(data, extension + 12)
            attributes_offset = extension + attribute_start
            attributes: list[Attribute] = []
            for index in range(attribute_count):
                attribute_offset = attributes_offset + index * attribute_size
                attribute_name_index = u32(data, attribute_offset + 4)
                raw_index = u32(data, attribute_offset + 8)
                data_type = data[attribute_offset + 15]
                value = u32(data, attribute_offset + 16)
                resource_id = (
                    resource_map[attribute_name_index]
                    if attribute_name_index < len(resource_map)
                    else 0
                )
                raw_value = (
                    strings[raw_index]
                    if raw_index != NO_INDEX and raw_index < len(strings)
                    else None
                )
                attributes.append(
                    Attribute(
                        resource_id=resource_id,
                        raw_value=raw_value,
                        data_type=data_type,
                        data=value,
                        data_offset=attribute_offset + 16,
                    )
                )
            name = strings[name_index] if name_index < len(strings) else f"#{name_index}"
            events.append(StartElement(name=name, attributes=tuple(attributes)))
        elif chunk_type == RES_XML_END_ELEMENT_TYPE:
            extension = position + header_size
            name_index = u32(data, extension + 4)
            name = strings[name_index] if name_index < len(strings) else f"#{name_index}"
            events.append(EndElement(name=name))
        position += chunk_size
    if position != len(data):
        raise ValueError("binary XML chunk walk did not end at EOF")
    return events


def attribute_string(attribute: Attribute | None, strings: list[str] | None = None) -> str | None:
    if attribute is None:
        return None
    if attribute.raw_value is not None:
        return attribute.raw_value
    return None


def extract_branding(data: bytes) -> Branding:
    events = parse_events(data)
    label_resource = 0
    icon_resource = 0
    first_theme = 0
    launcher_theme = 0
    launcher_activity: str | None = None
    current_activity: dict[str, object] | None = None
    activities: list[dict[str, object]] = []

    for event in events:
        if isinstance(event, StartElement):
            if event.name == "application":
                label = event.attribute(ANDROID_LABEL)
                icon = event.attribute(ANDROID_ICON)
                if label and label.data_type == TYPE_REFERENCE:
                    label_resource = label.data
                if icon and icon.data_type == TYPE_REFERENCE:
                    icon_resource = icon.data
            elif event.name in ("activity", "activity-alias"):
                name_attribute = event.attribute(ANDROID_NAME)
                theme_attribute = event.attribute(ANDROID_THEME)
                current_activity = {
                    "name": name_attribute.raw_value if name_attribute else None,
                    "theme": (
                        theme_attribute.data
                        if theme_attribute and theme_attribute.data_type == TYPE_REFERENCE
                        else 0
                    ),
                    "main": False,
                    "launcher": False,
                    "element": event.name,
                }
                if not first_theme and current_activity["theme"]:
                    first_theme = int(current_activity["theme"])
            elif current_activity is not None and event.name in ("action", "category"):
                name_attribute = event.attribute(ANDROID_NAME)
                value = name_attribute.raw_value if name_attribute else None
                if value == "android.intent.action.MAIN":
                    current_activity["main"] = True
                elif value == "android.intent.category.LAUNCHER":
                    current_activity["launcher"] = True
        elif isinstance(event, EndElement):
            if current_activity is not None and event.name == current_activity["element"]:
                activities.append(current_activity)
                current_activity = None

    launcher = next(
        (
            activity
            for activity in activities
            if activity.get("main") and activity.get("launcher")
        ),
        activities[0] if activities else None,
    )
    if launcher:
        launcher_theme = int(launcher.get("theme") or 0)
        launcher_activity = launcher.get("name") if isinstance(launcher.get("name"), str) else None

    if not label_resource or not icon_resource:
        raise ValueError(
            f"source application branding is incomplete: label=0x{label_resource:x}, icon=0x{icon_resource:x}"
        )
    if not launcher_theme:
        launcher_theme = first_theme
    if not launcher_theme:
        raise ValueError("source launcher theme resource was not found")
    return Branding(
        label_resource=label_resource,
        icon_resource=icon_resource,
        theme_resource=launcher_theme,
        launcher_activity=launcher_activity,
    )


def patch_branding(data: bytes, branding: Branding) -> tuple[bytes, dict[str, object]]:
    output = bytearray(data)
    replacements = {
        ANDROID_LABEL: (PLACEHOLDER_LABEL, branding.label_resource),
        ANDROID_ICON: (PLACEHOLDER_ICON, branding.icon_resource),
        ANDROID_THEME: (PLACEHOLDER_THEME, branding.theme_resource),
    }
    counts = {"label": 0, "icon": 0, "theme": 0}
    names = {ANDROID_LABEL: "label", ANDROID_ICON: "icon", ANDROID_THEME: "theme"}
    for event in parse_events(data):
        if not isinstance(event, StartElement):
            continue
        if event.name == "application":
            candidates = (ANDROID_LABEL, ANDROID_ICON)
        elif event.name == "activity":
            candidates = (ANDROID_THEME,)
        else:
            continue
        for resource_id in candidates:
            attribute = event.attribute(resource_id)
            if attribute is None:
                continue
            expected, replacement = replacements[resource_id]
            if attribute.data_type != TYPE_REFERENCE:
                raise ValueError(
                    f"branding attribute 0x{resource_id:08x} is not a reference"
                )
            if attribute.data != expected:
                raise ValueError(
                    f"branding placeholder mismatch for 0x{resource_id:08x}: "
                    f"expected 0x{expected:08x}, got 0x{attribute.data:08x}"
                )
            p32(output, attribute.data_offset, replacement)
            counts[names[resource_id]] += 1
    if counts != {"label": 1, "icon": 1, "theme": 1}:
        raise ValueError(f"unexpected branding patch counts: {counts}")
    verified = extract_branding(bytes(output))
    if verified != Branding(
        label_resource=branding.label_resource,
        icon_resource=branding.icon_resource,
        theme_resource=branding.theme_resource,
        launcher_activity="android.app.NativeActivity",
    ):
        raise ValueError(f"patched branding verification failed: {verified}")
    return bytes(output), {**branding.as_dict(), "patch_counts": counts}
