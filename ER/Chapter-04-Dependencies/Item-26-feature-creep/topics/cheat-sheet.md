# Item 26 · 记忆卡片

← [Item 26 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 原则 | Feature **只加能力**，互斥用 `cfg(target_*)` |
| 公开 API | **别** `#[cfg(feature)]` 门控 pub 字段/方法 |
| Unification | 全图**并集** — 用户控不了 |
| 命名 | 避 `no_*`；用 `std` 等正向名 |
| 数量 | 防 \(2^N\) — 少 feature + CI powerset |
| optional dep | 自动同名 feature |
