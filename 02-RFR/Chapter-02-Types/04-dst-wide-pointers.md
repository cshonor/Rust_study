# 1.4 Dynamically Sized Types and Wide Pointers（DST 与宽指针）

> 所属：**Types in Memory** · [← 章索引](./README.md)

## DST（动态大小类型）

编译期**大小未知**的类型，例如：

- `[T]`、`str`
- **`dyn Trait`**（trait object 的底层类型）

不能按值直接做栈上 `let x: str` 或 `let y: dyn Debug`——必须通过**指针 / 引用 / Box** 等间接持有。

## 宽指针 (Wide Pointer)

指向 DST 的引用在机器上常为 **两个指针宽**：

| 部分 | slice / str | `dyn Trait` |
|------|-------------|-------------|
| 数据指针 | 首元素地址 | 对象地址 |
| 元数据 | **长度** | **vtable 指针** |

因此 `&[T]`、`&str`、`&dyn Trait` 本身在编译期**大小固定**（两个 usize 或等价）。

## 与分发的关系

- `&dyn Trait` / `Box<dyn Trait>` → [05 编译与分发](./05-compilation-dispatch.md)
- `impl Trait` 返回具体类型、隐藏名字 → [10 存在类型](./10-existential-types.md)

ER → [Item 12 · dyn Trait 与 DST](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/07-dyn-trait-dst-carriers.md) · Book → [17.2 trait 对象](../../00-Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md)
