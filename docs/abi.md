# HyperOS Weather Native ABI

本文件记录 `libSnowWeatherRouter.so` 当前依赖的最小 ABI。来源是本地授权样本的静态与动态验证，仓库不包含原厂二进制。

## 全局状态

`RouterState`：

| 字段 | 用途 |
|---|---|
| activity | `ANativeActivity*` |
| runtime | `HYPER_FLUTTER_INTERFACE[0x08]` 返回值 |
| holder | `HYPER_FLUTTER_INTERFACE[0x40]` 返回值 |
| window | 当前 `ANativeWindow*` |
| resumed | lifecycle 状态 |

## HYPER_FLUTTER_INTERFACE

| 偏移 | 功能 |
|---:|---|
| 0x08 | runtime_create |
| 0x10 | runtime_destroy |
| 0x40 | create_shell_holder |
| 0x48 | destroy_shell_holder |
| 0x58 | shell_holder_launch |
| 0x60 | dispatch_platform_message |
| 0x78 | reply_platform_message |
| 0xA8 | surface_create |
| 0xB0 | set_window_size |
| 0xC0 | surface_destroy |

## 平台消息回调

```text
(context, channel_ptr, channel_len, reply_id, payload_ptr, payload_len)
```

reply：

```text
interface[0x78](holder, reply_ptr, reply_len, reply_id)
```

dispatch：

```text
interface[0x60](holder, channel, channel_len, payload, payload_len, 0)
```

## Engine 参数

- `--aot-shared-library-name=libapp.so`
- `--icu-symbol-prefix=_binary_icudtl_dat`
- `--impeller-backend=vulkan`
- entrypoint：`main`

## 已知 codec

- `PU`：JSONMethodCodec
- `RU`：StandardMethodCodec

新 router 使用完整通道字符串与方法名匹配，不再通过长度或抽样字节判断。
