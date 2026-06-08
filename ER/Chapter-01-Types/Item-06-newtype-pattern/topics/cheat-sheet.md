# Item 6 · 记忆卡片

← [Item 6 目录](../README.md)

| 要点 | 一句 |
|------|------|
| Newtype | 单字段 tuple struct = 新类型 |
| vs `type` | 别名不防混，Newtype 防混 |
| 孤儿 | 包成 local type 再 impl trait |
| `repr(transparent)` | 与内部同布局 |
| trait | 不继承，derive 或手写转发 |
