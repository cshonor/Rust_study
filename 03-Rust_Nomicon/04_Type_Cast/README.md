# 04 · Type Conversions

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（coercion / dot / cast / transmute）

---

## 一句话

**转换阶梯章** — 隐式 coercion → 点运算符 Deref/unsizing → `as` 显式 cast（非传递性）→ `transmute` 终极警告。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/coercions.rs](./src/coercions.rs) | 隐式弱化、`&dyn Trait` |
| [src/dot_operator.rs](./src/dot_operator.rs) | Auto-ref、Deref、unsizing 找方法 |
| [src/casts.rs](./src/casts.rs) | 数字 cast、指针 cast、非传递性 |
| [src/transmute.rs](./src/transmute.rs) | 同尺寸 reinterpret（含 UB 警示） |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/04_Type_Cast
cargo run
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| Deref coercion | [ER Item 05 demo](../../01-ER/Chapter-01-Types/Item-05-type-conversions/demo/) |
| 布局与 transmute | [RFR 08-casting](../../02-RFR/Chapter-09-Unsafe-Code/08-casting.md) |
| 上一章 | [03_Lifetime_Variance](../03_Lifetime_Variance/README.md) |
| 下一章 | [05_Uninit_Mem](../05_Uninit_Mem/README.md) · 未初始化内存 |

---

## 逻辑脉络

无害 coercion → 点运算符查找链 → `as` 显式边界 → transmute 禁区 → 进入 Uninitialized Memory。
