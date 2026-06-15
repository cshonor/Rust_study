# 第 1 章 · Welcome（Introduction） · §1.1 为什么要学这些（Why Learn This Stuff?）

← [本章目录](./README.md) · 上一节：[00-overview.md](./00-overview.md) · 下一节：[02-how-the-book-is-organized.md](./02-how-the-book-is-organized.md)

---

### 无处不在的小型语言（Little languages are everywhere）

- 世界不只有 C / Rust / Java 等**通用语言**。
- 大型项目里常有 **DSL**：配置格式、模板、查询语言、脚本层……
- 找不到现成库时，**自己写解析器**是极其实用的技能。

**本仓库联想**：`00-Book` 宏、`build.rs`、Cargo.toml、`#[derive]` 背后都是「小语言 + 处理器」；RFR 第 7 章宏也是同一谱系。

### 绝佳的编程锻炼（Languages are great exercise）

实现一门语言是对编程能力的**终极综合练习**，逼你真正会用：

| 数据结构 / 技巧 | 在解释器里典型出现 |
|-----------------|-------------------|
| 递归 | 解析、树遍历 |
| 动态数组 | Token 流、字节码 chunk |
| 树 | AST |
| 图 | 控制流、依赖（后文 clox 更多） |
| 哈希表 | 符号表、全局/局部绑定、字符串驻留 |

**与 RFR**：第 2 章 layout、第 1 章内存区域——读 clox 时会反复碰到。

### 还有一个原因（One more reason）

- 作者曾把写语言的人当成掌握魔法的「**巫师**」。
- 本章态度：**没有魔法**——底层只是一行行代码；做语言的人也是普通人。
- 目的：破除对编译器 / 解释器 / 「底层」的**胆怯**。

读 **04 LLVM** IR 或 **Nomicon** 时若发怵，可回到 §1.1 这句。

---
