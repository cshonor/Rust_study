# 第 5 章 · 迭代器

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 4 章 基本类型](../chapter04_primitive_types/README.md) · 后：[第 6 章 基本类型（续）](../chapter06_basic_types_continued/README.md) · 原书目录：[本书目录 § 第 5 章](../本书目录.md#第-5-章--迭代器)

**本章定位**：`Iterator` trait、各集合上的迭代器实现、适配器链与 `Option` 迭代 — 零成本抽象在 `libstd` 中的核心体现。

**阅读顺序**：**5.1 → 5.2 → … → 5.9**

---

## 子节索引

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **5.1** | 三种迭代器 | 📝 规划 |
| **5.2** | Iterator Trait 分析 | 📝 规划 |
| **5.3** | Iterator 与其他集合类型转换 | 📝 规划 |
| **5.4** | 范围类型迭代器 | 📝 规划 |
| **5.5** | 切片类型迭代器 | 📝 规划 |
| **5.6** | 字符串类型迭代器 | 📝 规划 |
| **5.7** | 数组类型迭代器 | 📝 规划 |
| **5.7.1** | 成员本身迭代器 | 📝 规划 |
| **5.7.2** | 成员引用迭代器 | 📝 规划 |
| **5.8** | Iterator 适配器 | 📝 规划 |
| **5.8.1** | Map 适配器 | 📝 规划 |
| **5.8.2** | Chain 适配器 | 📝 规划 |
| **5.8.3** | 其他适配器 | 📝 规划 |
| **5.9** | `Option<T>` 适配器 | 📝 规划 |

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| Iterator / 适配器 | [2.4 闭包与 Iterator](../chapter02_rust_features_summary/2.4-closures-iterator-in-stdlib.md) |
| 语法底座 | [Book 13 闭包与迭代器](../../00-Book/13-iterators-closures/) |
| 源码 | `core::iter` · `library/core/src/iter/` |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **5.8** | `map`/`filter`/`take` 处理 tick 流；注意内联与分支预测 |
| **5.5～5.7** | 切片 / 定长数组遍历订单档、ring buffer 槽位 |
| **5.9** | `and_then` / `transpose` 减少嵌套 `match` |
