# Item 25 · 记忆卡片

← [Item 25 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 解析 | toml 范围 → lock 固化 |
| App lock | **要提交** |
| Lib lock | **别提交**给下游 |
| Features | 同版本取**并集** |
| 多版本 | Rust OK；**FFI 不行** |
| 工具 | tree / udeps / deny |
| 版本 | `"1"` 或 `"1.4.23"`，禁 `*` |
