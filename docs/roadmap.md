# Roadmap

## 已完成

- 新主库 `libSnowWeatherRouter.so`，不修改旧 bridge；
- Rust/LLD 无完整 NDK构建；
- NativeActivity lifecycle / Surface / HyperOS Flutter engine 启动；
- JSONMethodCodec 与 StandardMethodCodec 基础路由；
- package_info、system properties、SharedPreferences、DeviceLevel 等启动期契约；
- 每条平台通道与 payload 输出到 `logcat`；
- GitHub Release 自动构建未签名 APK。

## 后续

- 根据 Snow 日志补全剩余平台通道的真实返回值；
- 可选最小 DEX 层：权限请求、Activity Result、Service/Receiver；
- 多 ABI 支持；
- 可选用户自有 keystore 的 GitHub Actions 签名；
- 自动识别不同 Weather 版本的包名与 ABI drift。
