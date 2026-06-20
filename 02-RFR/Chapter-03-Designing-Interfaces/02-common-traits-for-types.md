# 1.2 Common Traits for Types（类型的通用 Trait）

> 所属：**Unsurprising** · [← 章索引](./README.md)

← [01 命名](./01-naming-practices.md) · 下一节 [03 人体工程学 impl](./03-ergonomic-trait-implementations.md)

用户期望公开类型在常见场景下「开箱即用」。

---

## 几乎总是该有

| Trait | 理由 |
|-------|------|
| **`Debug`** | 日志、测试、`{:?}`；哪怕实现较简也应提供 |
| **`PartialEq` / `Eq`** | 测试断言、`HashMap` 键（配合 `Hash`） |

## 异步 / 多线程默认假设

| Trait | 说明 |
|-------|------|
| **`Send`** | 可跨线程转移所有权 |
| **`Sync`** | `&T` 可跨线程共享 |

若**不能**实现，应在文档中**明确原因与替代用法**（例如 `Rc` 非 `Send`）。

## 谨慎对待

| Trait | 注意 |
|-------|------|
| **`Copy`** | 一旦加上，未来很难引入 `String` 等非 `Copy` 字段；也可能带来隐式复制的性能/语义意外。仅「稳定的小值类型」优先考虑。 |
| **`Hash`** | 与 `Eq` 一致；可变字段作键时需格外小心 |

---

→ **完整解读 + 代码示例 + 五套可复制模板**：[02-1 通用标准 Trait 完整解读](./02-1-common-traits-full-guide.md) · [02-1 速记](./02-1-cheat-sheet.md) · [02 速记](./02-cheat-sheet.md)

ER → [Item 10 标准 trait](../../01-ER/Chapter-02-Traits/Item-10-standard-traits/README.md) · Book → [10.2 trait · derive](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md)
