# 3.5 · Drop Check · 速记

← [09 Drop 检查](./09-drop-check.md) · [07 Panic](./07-panics.md) · [章索引](./README.md)

---

## dropck 是什么

带 `impl Drop` 的泛型类型 — 析构时不能访问已失效借用。

## 黄金规则

泛型参数（`'a`、`T`）须 **长于** `Self` 存活。

## 不向 dropck 强加内部析构

`ManuallyDrop<T>` · `MaybeUninit<T>` · `PhantomData`（声明用）

## unsafe 绕过/放宽

| 工具 | 作用 |
|------|------|
| `ManuallyDrop` | 不自动 drop；手动 `drop()` |
| `forget` | 永不 drop（泄漏） |
| `#[may_dangle]` | 担保 drop 不碰过期借用 |
| `Pin` | 禁移 + 自引用 |

## Drop 误用

破损状态析构 · 双重 drop · drop 内 panic

## 顺序

struct 字段：**逆声明** · Vec 元素：**正序** → [Ch01 04.4](../Chapter-01-Foundations/04-4-drop-order.md)

## 自测

- [ ] `Inspector<'a>` + `drop(vec)` 为何编译失败？  
- [ ] `ManuallyDrop` 为何能改变 dropck 约束？  
- [ ] `Unpin` 与 dropck 有何不同？
