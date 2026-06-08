# Item 10 · 记忆卡片

← [Item 10 目录](../README.md)

| Trait | 一句 |
|-------|------|
| `Copy` | 小纯数据；改 Move→按位拷 |
| `Debug` / `Display` | `:?` 可 derive；`{}` 手写 |
| `Eq` | 标记 + 反射；浮点不行 |
| `Hash` + `Eq` | 相等则 hash 相等 |
| `Ord` derive | 字段顺序 ≠ 业务优先级 |
