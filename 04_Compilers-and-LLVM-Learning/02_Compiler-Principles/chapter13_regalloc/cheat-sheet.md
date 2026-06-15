# 第 13 章 · 寄存器分配 · 速记与自测

← [本章目录](./README.md) · 上一节：[05-advanced-topics.md](./05-advanced-topics.md)

---

## 本章速记

```text
§1  reg≪mem · spill · Allocation vs Assignment · 寄存器类
§2  块内 自顶向下/自底向上 · next-use
§3  LIVE · Live Range · 跨块冲突边
§4  冲突图 k-着色 · spill 代价 · Coalescing 删 copy
§5  预着色/机器约束 · splitting · Part IV 收官
```

---

## 三句背诵

1. **Regalloc = 虚拟寄存器映射到 k 个物理寄存器，不够就 spill。**
2. **冲突图：同时活跃不能同色；k-着色启发式 + spill 代价。**
3. **Coalesce 不冲突的 copy 对 — 同色消 mov。**

---

## 后端三关（全书）

| 章 | 任务 |
|:--:|------|
| 11 | 选指令 |
| 12 | 排顺序 |
| 13 | 分寄存器 |

---

## 自测

- [ ] Allocation 与 Assignment
- [ ] Live range 与冲突边
- [ ] 简化栈着色四步概念
- [ ] Coalescing 条件
- [ ] ch12 与 ch13 如何互相影响

---

## 阅读进度

- [x] ch13 寄存器分配 — **Part IV 完成 · 正文 13 章完结**
- [ ] 附录 A · B
