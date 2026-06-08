# Item 3 · 记忆卡片

← [Item 3 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 风格 | 能链式 / `?` 就少 `match` |
| API | 有原因用 `Result`，别全变 `Option` |
| 性能 | 转换方法可内联，不必怕链式 |
| 引用 | `&Option<T>` → 先 `as_ref()` |
| unwrap | 失败即 panic，库代码慎用 |
