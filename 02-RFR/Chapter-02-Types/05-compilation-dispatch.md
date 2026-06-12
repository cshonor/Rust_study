# 2.1 Compilation and Dispatch（编译与分发）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

Trait 描述不同类型之间的**可替换行为**与**互操作契约**。编译器如何把方法调用落到机器码，是本章核心之一。

## 静态分发（单态化 / Monomorphization）

- 对每个具体类型 `T` 生成一份特化代码。
- **优点**：易内联、峰值性能好。
- **缺点**：编译时间、产物体积可能上升。

## 动态分发 (`dyn Trait`)

- 通过 **vtable** 在运行时解析方法地址。
- **优点**：少份代码、编译更快、二进制更省（相对）。
- **缺点**：间接调用开销；需 [04 DST 与宽指针](./04-dst-wide-pointers.md) 载体。

## 对象安全 (Object Safety)

并非所有 Trait 都能写成 `dyn Trait`。常见阻碍：

- 泛型方法
- 返回 `Self` 且与对象大小绑定
- 需要 `Self: Sized` 的方法等

无法在 vtable 中统一表达 → **非对象安全**。

## 选型速记

| 场景 | 倾向 |
|------|------|
| 泛型参数 `T: Trait` | 静态分发 |
| 异构集合、运行时选实现 | `dyn Trait` |
| 返回闭包 / 长迭代器链 | `impl Trait` → [10 存在类型](./10-existential-types.md) |

ER → [Item 12](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) · [06-dispatch-beginner-guide](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/06-dispatch-beginner-guide.md)
