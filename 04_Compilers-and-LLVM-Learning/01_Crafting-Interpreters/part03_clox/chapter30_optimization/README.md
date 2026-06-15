# 第 30 章 · Optimization（优化）

> **Crafting Interpreters** · [Part III · clox](../README.md) · [本书目录](../../本书目录.md)  
> 在线：[craftinginterpreters.com](https://craftinginterpreters.com/optimization.html) · [中文在线](https://craftinginterpreters.com/optimization.html)

## 状态

- [x] 已读（笔记整理）

---

## 一句话

全书 Lagniappe（额外赠礼） · 最后一章含新代码。用 Benchmark + Profiler 找热点，实施两项 底层微优化 —— 不改 Lox 语义，只压榨 VM 实现。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §30.1～ | §30.2 哈希表探测优化（Hash Table Probe Optimization） | [01-hash-table-probe-optimization.md](./01-hash-table-probe-optimization.md) |
| §30.3 | NaN 装箱（NaN Boxing） | [02-nan-boxing.md](./02-nan-boxing.md) |
| ·4 | 优化方法论（全书收束） | [03-optimization.md](./03-optimization.md) |
| ·5 | 全书 30 章能力地图 | [04-optimization-30.md](./04-optimization-30.md) |
| — | 速记 · 自测 · 进度 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 逻辑脉络

按上表 **§ 顺序** 阅读；`cheat-sheet.md` 含速记与自测。
