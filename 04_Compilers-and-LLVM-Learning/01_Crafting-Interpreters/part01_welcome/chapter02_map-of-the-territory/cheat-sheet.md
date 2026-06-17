# 第 2 章 · A Map of the Territory（领域地图） · 速记与自测

← [本章目录](./README.md) · 上一节：[03-a-map-of-the-territory.md](./03-a-map-of-the-territory.md)

---

## 本章速记

```text
§2.1  扫描→Token · 解析→AST · IR · 代码生成 · VM/字节码 · Runtime/GC
§2.2  单遍（省 AST）· 树遍历（jlox）· 转译（换高级语言）
全书  = 先 §2.2.2 再 §2.1.7 的 clox 路线
Rust  = 源码→AST→LLVM IR→机器码→ELF（见 04-rust-hft-编译流水线对照.md）
LLVM≠Runtime = 编译期翻译 vs 运行期调度（见 05-compile-time-llvm-vs-runtime.md）
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **3** | [chapter03 · The Lox Language](../../chapter03_the-lox-language/) | **Lox 语言规格**——动手写 jlox 前必读 |
| **4** | Part II · Scanning | 流水线第一站：Token |

---

---

## 自测 / 对照（可选）

- [ ] 画一条从「Rust 源码」到「CPU 执行」的简化 pipeline（可含 `rustc` → LLVM IR → 机器码）。
- [ ] 说明 **jlox** 在 §2.1 的哪几站停下、**clox** 多走了哪几站。
- [ ] 各举 1 个 **transpiler** 与 **VM 语言** 的例子。
- [ ] 对照 RFR [03-2 OS/LLVM 内存布局](../../../../02-RFR/Chapter-01-Foundations/03-2-os-memory-layout.md)：`alloca`（栈）vs heap 分别更像 pipeline 哪一段的产物？
- [ ] 一句话区分 **LLVM**（编译期）与 **Runtime**（运行期）；Tokio 属于哪一类 runtime？

---

---

## 阅读进度

- [x] §2.1～§2.2 结构梳理（本章笔记）
- [ ] 本章 Challenges（若有）
- [ ] 进入 ch03 Lox 规格
