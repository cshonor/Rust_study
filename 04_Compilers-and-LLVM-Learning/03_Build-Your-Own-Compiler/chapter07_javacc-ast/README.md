# 第 7 章 · JavaCC 的 action 和抽象语法树

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 第2部分 · 抽象语法树和中间代码

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**第2部分开篇** — 语法规则只 **检查**；**action `{}`** 在匹配时跑 Java 代码 **建 AST**；**Token** 语义值 · `return` 非终端值；**`ast` 包 Node 层次**（`AST`/`StmtNode`/`ExprNode`…）· `location()`/`dump()`；**手写节点** 弃 JJTree 以保 **静态类型**。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | JavaCC 的 action | [01-javacc-actions.md](./01-javacc-actions.md) |
| §2 | 抽象语法树和节点 | [02-ast-and-nodes.md](./02-ast-and-nodes.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch7 | 对照 |
|----------|------|
| 第1部分 ch6 | [完整文法](../chapter06_parsing/README.md) — 本章 **填 action** |
| ch8 下一章 | 各类 AST 节点实现细节 |
| ch2 `ast` 包 | [cbc 包结构](../chapter02_cflat-cbc/02-cbc-packages.md) |
| CI / EaC | [CI 表示代码](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/) · [EaC ch5 IR](../../../02_Compiler-Principles/chapter05_ir/) |

---

## 逻辑脉络

action 机制 → Node 类群与 cbc 设计取舍。
