# Item 19 · 记忆卡片

← [Item 19 目录](./README.md)

| 要点 | 一句 |
|------|------|
| Rust | **无**完整反射 / RTTI |
| 设计 | **Trait + 宏**，别 runtime 内省 |
| `type_name` | 调试用；别写业务逻辑 |
| `TypeId` | 要稳定 ID 用这个 |
| `Any` | 稀有逃生舱；仅 downcast 原类型 |
| `'static` | Any 故意不要带借用的 T |
