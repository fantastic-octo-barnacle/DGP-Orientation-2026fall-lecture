# Rust 客户端—服务端通信协议

本协议用于 Rust 起点路线的参考程序、客户端和服务端。协议刻意保持简单，重点考查 TCP 字节流、错误处理和不同连接之间的异步并发。

## 传输

- 仅监听本机地址，默认 `127.0.0.1:7878`。
- 使用 UTF-8。
- 每行一个完整 JSON 消息，接受 `\n` 和 `\r\n`。
- 单条编码后消息最大 64 KiB。
- 每个请求产生一个响应。
- 同一连接内依次处理请求，不要求请求多路复用。
- 不同连接可以并发处理。

## 请求

`id` 为非负整数。

```json
{"id":1,"action":"ping"}
{"id":2,"action":"echo","data":"hello"}
{"id":3,"action":"delay","milliseconds":1000,"data":"hello"}
```

动作：

- `ping`：立即返回 `pong`。
- `echo`：返回字符串 `data`。
- `delay`：等待指定毫秒数后返回字符串 `data`；允许范围为 0–10,000 毫秒。

## 响应

成功响应包含请求 `id`、`ok: true` 和结果数据：

```json
{"id":1,"ok":true,"data":"pong"}
```

错误响应包含可用的请求 `id`、`ok: false`、错误代码和说明。无法解析请求标识时，`id` 使用 `null`。

```json
{"id":null,"ok":false,"error":{"code":"invalid_json","message":"request is not valid JSON"}}
```

错误代码至少包括：

- `invalid_json`
- `invalid_request`
- `unknown_action`
- `line_too_long`

具体错误行为：

| 情况 | 行为 |
|---|---|
| 非法 JSON | 返回 `invalid_json`，继续连接 |
| 缺少字段、类型错误或负数 ID | 返回 `invalid_request`，继续连接 |
| 未知动作 | 返回 `unknown_action`，继续连接 |
| `delay` 超出范围 | 返回 `invalid_request`，继续连接 |
| `echo` 或 `delay` 的 `data` 不是字符串 | 返回 `invalid_request`，继续连接 |
| 存在未声明字段 | 返回 `invalid_request`，继续连接 |
| 单行超过 64 KiB | 尽量返回 `line_too_long`，然后关闭该连接 |
| 客户端正常 EOF | 正常关闭该连接，不视为服务端故障 |

所有协议错误都不得导致整个服务端退出。

## 核心异步验收

1. 客户端 A 发送 `delay`，等待 5 秒。
2. 等待期间，客户端 B 发送 `ping`。
3. B 应立即获得响应。
4. A 随后正常获得响应。
5. A 异常断开不影响 B 或服务端进程。

不要求同一客户端同时存在多个未完成请求，也不要求跨连接响应顺序。

参考程序的具体行为见[参考程序规范](./rust-reference-program.md)。
