# Item 18 · 记忆卡片

← [Item 18 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 默认 | **`Result` > `panic!`** |
| panic 用途 | 不可恢复 bug / 顶层 `main` |
| catch_unwind | 不是 try-catch；别用来「修」业务错误 |
| abort | 配置后 catch 无效 |
| 文档 | 公共 API 写 `# Panics` |
| 隐性 | unwrap、越界、除零 — 用机器查 |
