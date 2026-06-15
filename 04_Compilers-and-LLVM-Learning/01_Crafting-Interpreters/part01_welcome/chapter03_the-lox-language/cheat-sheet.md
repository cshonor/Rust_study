# 第 3 章 · The Lox Language（Lox 语言概览） · 速记与自测

← [本章目录](./README.md) · 上一节：[11-lox.md](./11-lox.md)

---

## 本章速记

```text
C 系语法 · 动态类型 · GC
类型：bool / number / string / nil
表达式有值 · 语句有副作用 · and/or 短路
var · if/while/for · fun 一等 + 闭包
class / this / init · 单继承 < · super
标准库只有 clock()
```

---

---

## 读后下一步

Part I 读完 → **Part II 开始写代码**

| 章 | 目录 | 内容 |
|:--:|------|------|
| **4** | [chapter04 · Scanning](../../part02_jlox/chapter04_scanning/) | 流水线第一站：**扫描器** → Token |

建议：写 jlox 前扫一眼 **附录 I** 语法；实现中遇歧义回查本章 + A1。

---

---

## 自测 / 对照（可选）

- [ ] 写 3 行 Lox 代码：含 `var`、闭包、`class` 与 `init` 各一。
- [ ] 解释：`and` 短路为何算「控制流味道」？
- [ ] 对比 Rust：`Lox 动态类型` vs `Rust 静态 + Option` 在**何时**发现类型错误。
- [ ] 说明 Lox **无 `new`** 实例化与 Java/C++ 的差异。

---

---

## 阅读进度

- [x] §3.1～§3.10 结构梳理（本章笔记）
- [ ] Design Note *Expressions and Statements*
- [ ] 附录 I Lox Grammar（写 ch4 前建议）
- [ ] Part I 完成 → 进入 **ch4 Scanning**
