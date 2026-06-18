# 宏整体分类 · 速记

← [00 宏分类总览](./00-macro-taxonomy.md) · [章索引](./README.md)

---

## 两大体系

**声明宏** `macro_rules!` · **过程宏** 3 类（derive / attr / `foo!`）

## 撞脸警告

`xxx!()` = 声明宏 **或** 类函数过程宏 · **看定义，不看 !**

## 四类形态

| 类型 | 形态 |
|------|------|
| 声明 | `foo!(...)` + `macro_rules!` |
| Derive | `#[derive(T)]` |
| Attribute | `#[attr(...)]` 项上 |
| Function-like proc | `foo!(...)` + proc-macro crate |

## 声明 vs 类函数 proc

模板 Token 匹配 · 简单 · 本 crate  
vs  
Rust 程序 · DSL/SQL · 独立 proc-macro

## 判断四步

`macro_rules!` · proc crate + `!` · `derive` · `#[attr]`

## 选型口诀

简单模板 → 声明 · impl 批量 → derive · 改整项 → attr · DSL → proc `!` · 能泛型 → 不用宏

## 自测

- [ ] `vec!` 和 `json!` 分别是哪类宏？  
- [ ] 为何不能靠 `!` 区分声明宏与过程宏？  
- [ ] `#[tokio::main]` 属于哪一类？
