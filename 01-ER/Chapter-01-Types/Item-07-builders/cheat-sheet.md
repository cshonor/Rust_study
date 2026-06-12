# Item 7 · 记忆卡片

← [Item 7 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 为何 Builder | 多可选字段、无全局 Default |
| 消费型 | 好链式；if 要重绑；build 一次 |
| 借用型 | 好分支；先 `let mut builder` 再改 |
| 宏 | 优先 `derive_builder` |
| 校验 | 放在 `build()` |
