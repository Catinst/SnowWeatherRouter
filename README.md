# SnowWeatherRouter

带 **Snow / Snownight 水印**的 HyperOS Weather 原生路由器与可复现 APK 转换流水线。

私有仓库：<https://github.com/Catinst/SnowWeatherRouter>

## 设计

本项目不再修改原来约 5 KiB 的 `libdatastore_shared_counter.so` 代码区。

输出 APK 会：

- 保留原 APK 中的旧 `libdatastore_shared_counter.so`，不修改它；
- 新增从 Rust 源码编译的 `libSnowWeatherRouter.so`；
- 将 Manifest 改为 `android.app.NativeActivity`；
- 将 `android.app.lib_name` 改为 `SnowWeatherRouter`；
- 保留 Flutter AOT、资源与其他原厂 SO；
- 对等长包名进行安全的 ASCII/UTF-16 原位替换；
- 删除旧 APK 签名痕迹；
- 输出未签名 APK，交由 MT Root 或用户自己的签名流程安装。

`libSnowWeatherRouter.so` 内含导出水印：

- `SNOW_WEATHER_ROUTER_WATERMARK`
- `SNOW_WEATHER_ROUTER_BUILD`
- 文本：`SnowWeatherRouter|Snownight|v1`

## 为什么默认不补 DEX

当前启动路径是 `NativeActivity + hasCode=false`。平台消息 messenger 与 HyperOS Flutter
引擎入口都位于 native ABI 中。单独增加 DEX 无法接管该消息链，仍需要 native router。
因此首版直接让系统加载新的 Snow SO，结构更短、可审计、无固定空间限制。
后续如需 Java 权限代理、Activity Result、Service/Receiver，再增加最小 DEX 层。

## 仓库不包含

- 原始或修改后的 APK；
- Xiaomi/HyperOS/Flutter 原厂 SO；
- 签名证书、keystore、token；
- 本地逆向输出、虚拟环境、编译缓存。

所有原厂二进制均由使用者在本地或私有 GitHub Release 中提供。

## 本地构建

要求：

- Python 3.11+
- `pip install -r requirements.txt`
- Rust 1.98.0（由 `rust-toolchain.toml` 固定），安装 `aarch64-linux-android` target
- Android SDK Build Tools 与任一 Android Platform（需 `aapt2` 和 `android.jar`）

```bash
rustup target add aarch64-linux-android
python -m pip install -r requirements.txt
python scripts/build_apk.py /path/to/MIUIWeather.apk \
  --sdk "$ANDROID_SDK_ROOT" \
  --target-package com.miui.weather3 \
  --output dist/MIUIWeather-Snow-unsigned.apk
```

Windows 示例：

```powershell
python scripts/build_apk.py E:\APK\MIUIWeather.apk `
  --sdk "$env:LOCALAPPDATA\Android\Sdk" `
  --target-package com.miui.weather3 `
  --output dist\MIUIWeather-Snow-unsigned.apk
```

包名替换要求源包名与目标包名长度相同，例如：

- `com.miui.weather2` → `com.miui.weather3`

## GitHub Release 自动构建

1. 在仓库中创建 **Draft Release**；
2. 上传你有权处理的原 APK；
3. 发布 Release；
4. `Build Snow APK from Release` Action 自动：
   - 下载 Release 中第一个非 Snow 的 APK；
   - 编译 `libSnowWeatherRouter.so`；
   - 转换 APK；
   - 将 `*-Snow-unsigned.apk` 上传回同一 Release；
   - 同时保存为 Actions Artifact。

也可在 Actions 页面手动运行，并指定 Release tag、asset 名和目标包名。

## 重要限制

- 输出默认未签名；
- 小米安全中心可能拒绝普通 `adb install`，此时使用用户自己的 MT Root/系统安装方式；
- 本项目目前针对 arm64 HyperOS Flutter Weather ABI；
- 新系统版本若 ABI 改变，应先更新 `docs/abi.md` 和 router 接口表；
- 仅处理你拥有或获授权处理的 APK。
