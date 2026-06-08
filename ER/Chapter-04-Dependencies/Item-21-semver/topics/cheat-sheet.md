# Item 21 · 记忆卡片

← [Item 21 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 三段 | MAJOR break / MINOR 加功能 / PATCH 修 bug |
| `0.x` | 左移规则：最左非零位变 = break |
| 作者 | 少暴露、敢 1.0、deprecated 过渡 |
| 用户 | 禁 `*`、定期规划 MAJOR |
| `^1.2.3` | 默认：`<2.0.0` |
| lock | 库 lock 不保护下游；CI 要自动更新 |
