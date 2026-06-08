# Item 32 · 记忆卡片

← [Item 32 目录](../README.md)

| 要点 | 一句 |
|------|------|
| 目的 | 自动化全书最佳实践 |
| 确定性 | **`rust-toolchain.toml`** |
| 节奏 | PR 快检 + 定期重检 + fuzz 后台 |
| 铁律 | **全绿**、无 flaky |
| 本地 | CI 命令 = 开发者能先跑 |
| 开源 | 限 fork CI、钉 Action SHA |
| bench | CI 结果**仅供参考** |
