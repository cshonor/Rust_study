# Item 30 · 记忆卡片

← [Item 30 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 单元 | 同文件，可测 private |
| 集成 | `tests/`，仅 pub API |
| doc / examples | 文档测 + 用户向 demo |
| fuzz | 不可信输入 **必做** |
| bench | **`black_box`** / criterion |
| 修 bug | **先写 failing test** |
| 测试 vs 示例 | 测试可 unwrap；**examples 用 `?`** |
