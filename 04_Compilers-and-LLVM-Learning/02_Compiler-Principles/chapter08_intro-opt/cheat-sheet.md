# 第 8 章 · 代码优化概述 · 速记与自测

← [本章目录](./README.md) · 上一节：[05-interprocedural-opt.md](./05-interprocedural-opt.md)

---

## 本章速记

```text
§1  Local→Superlocal→Regional→Global→Whole-program · 代价↑机会↑
§2  块内 DAG · 值编号 VN · 消冗余
§3  超局部 VN · 支配者 VN · 跨块
§4  可用表达式 · 全局 CSE · ch9 数据流
§5  内联 · 复制/特化 · LTO · Part III 开篇
```

---

## 三句背诵

1. **优化按作用域分层；块内 VN，全局靠数据流。**
2. **可用表达式：问「这式子前面算不算过且操作数没变」。**
3. **内联换体积，换上下文，换更多全局优化机会。**

---

## 与仓库对照

| 橡书 ch8 | 本仓库 |
|----------|--------|
| 可用表达式/ch9 | 04 LLVM Pass |
| VN/GVN | O0 vs O3 IR diff |
| 局部优化直觉 | clox ch30 |

---

## 自测

- [ ] 五层作用域各一句
- [ ] DAG 与 VN 各解决什么
- [ ] 可用表达式分析解决什么问题
- [ ] 内联的收益与代价

---

## 阅读进度

- [x] ch8 优化概述
- [ ] ch9 数据流分析
