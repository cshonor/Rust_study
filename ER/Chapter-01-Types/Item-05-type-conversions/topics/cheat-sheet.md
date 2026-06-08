# Item 5 · 记忆卡片

← [Item 5 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 数值 | 无隐式转换，必须显式 |
| 实现 | 只写 `From` |
| 边界 | 用 `Into` |
| 可能失败 | `TryFrom` + `Result` |
| `as` | 可有损；优先 trait 转换 |
| Coercion | Deref、切片、trait 对象等少数自动 |
