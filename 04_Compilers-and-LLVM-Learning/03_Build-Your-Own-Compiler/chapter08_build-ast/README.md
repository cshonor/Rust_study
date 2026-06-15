# 第 8 章 · 抽象语法树的生成

> **《自制编译器》** · [03 Build Your Own Compiler](../../README.md) · [本书目录](../../本书目录.md) · 第2部分 · 抽象语法树和中间代码

## 状态

- [x] 已读（笔记整理）

---

## 一句话

**AST 落地章** — 自下而上在 `.jj` **action** 里 `new` 节点：**表达式**（literal/`TypeRef`·`Type` · 左结合 `BinaryOpNode*` · 右结合 `AssignNode` 递归）→ **语句**（If/While/Block）→ **声明**（`DefinedVariable` 列表 · `DefinedFunction` · **`AST` 根** · `import` 读 `.hb`）→ **`Parser.parse()`** 启动。

---

## 专项笔记（一小节一文件）

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| §1 | 表达式的抽象语法树 | [01-expr-ast.md](./01-expr-ast.md) |
| §2 | 语句的抽象语法树 | [02-stmt-ast.md](./02-stmt-ast.md) |
| §3 | 声明的抽象语法树 | [03-decl-ast.md](./03-decl-ast.md) |
| §4 | cbc 解析器的启动 | [04-parser-startup.md](./04-parser-startup.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

---

## 与仓库其他部分

| 本书 ch8 | 对照 |
|----------|------|
| ch6 文法 | [chapter06_parsing](../chapter06_parsing/README.md) |
| ch7 action/Node | [chapter07_javacc-ast](../chapter07_javacc-ast/README.md) |
| ch9 语义 | 引用消解 — 遍历本章产出的 AST |
| CI | [表示代码 · AST](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/) |

---

## 逻辑脉络

primary → expr → stmt → 声明/AST 根 → Parser 入口。
