# 3. Existential Types（存在类型）

> [← 章索引](./README.md)

## `impl Trait` 是什么

在**返回位置**（及参数位置等）向调用方承诺：

> 这里有一个**具体类型**满足某 `Trait`，但**不暴露其名字**。

- 仍是**静态分发**（编译器知道具体类型，可单态化 / 内联）。
- 与 `dyn Trait`（动态分发 + 宽指针）不同。

## 典型场景

- 闭包组合、长 **迭代器链**（具体类型名极长）
- **`async fn`** 返回的 `Future` 类型
- 隐藏实现细节，稳定 API 表面

## 与 trait object 的取舍

| | `impl Trait` | `dyn Trait` |
|---|--------------|-------------|
| 分发 | 静态 | 动态 |
| 大小 | 编译期已知 | 需间接（DST） |
| 异构集合 | 不行（需 enum 或 `dyn`） | 可以 `Vec<Box<dyn Trait>>` |

## 存在量化（进阶）

- `impl Trait` ≈ 存在某类型 `∃T. T: Trait`
- HRTB `for<'a>` ≈ 全称量化 `∀'a`

Book → [10.2.3 impl Trait 全解](../../00-Book/10-generics-traits-lifetimes/10.2.3-impl-Trait全解.md) · ER → [Item 12](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md)
