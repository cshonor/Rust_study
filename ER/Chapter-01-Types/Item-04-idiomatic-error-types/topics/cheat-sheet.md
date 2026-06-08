# Item 4 · 记忆卡片

← [Item 4 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 惯例 | 自定义 `E` 应 `Display + Debug + Error` |
| 孤儿 | 不能 `impl Error for String`；用 Newtype / enum |
| 库 | 具体 `enum` + `thiserror` |
| 应用 | `anyhow` / `Box<dyn Error>` |
| `?` | `From` 子错误，少 `map_err` |
| `source` | 链式暴露底层原因 |
