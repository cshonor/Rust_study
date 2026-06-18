# 2.1 · 过程宏三子类 · 速记

← [04 过程宏类型](./04-types-of-procedural-macros.md) · [07 如何工作](./07-how-procedural-macros-work.md) · [章索引](./README.md)

---

## 三类（全是 proc-macro）

| 子类 | 语法 | 作用 |
|------|------|------|
| **Derive** | `#[derive(T)]` | 追加 `impl` |
| **Attribute** | `#[attr(...)]` 项上 | 改写/包装目标项 |
| **Function-like** | `foo!(...)` | 自定义 Token / DSL |

## 共性

`TokenStream → TokenStream` · 独立 proc-macro crate · 编译期运行 Rust

## 易混

`vec!` `format!` = **声明宏** · `json!` `query!` = **过程宏**（同为 `!` 语法）

## 入口属性

`proc_macro_derive` · `proc_macro_attribute` · `proc_macro`

## 自测

- [ ] `#[derive(Serialize)]` 属于哪类？输入是什么 Token？  
- [ ] `vec!` 是过程宏还是声明宏？  
- [ ] 属性宏比 derive 多接收哪一部分输入？
