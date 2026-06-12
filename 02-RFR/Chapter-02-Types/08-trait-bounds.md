# 2.4 Trait Bounds（Trait 限定）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

Trait bound 把「此类型必须支持的行为」写进签名，供编译期检查与单态化。

## 常见写法

```rust
fn f<T: Debug + Clone>(x: T) { ... }
fn g(x: impl Display) { ... }
fn h(x: &dyn Error) { ... }
```

- **`T: Trait`**：泛型约束，静态分发。
- **`impl Trait`**（参数位置）：语法糖 + 存在类型语义。
- **`&dyn Trait`**：trait object，动态分发。

## 高阶 Trait 限定 (HRTB)

语法：`for<'a> ...` ——「对**任意**生命周期 `'a`，某约束都成立」。

**典型场景**：函数要接受**任意生命周期**的引用，例如：

```rust
fn takes_fn<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{ ... }
```

若写成 `F: Fn(&'a str)` 且 `'a` 来自外部，则过窄；HRTB 表达「闭包对任何 `'a` 都合法」。

## 与生命周期的关系

- HRTB 常出现在高阶闭包、回调 API。
- 与 RFR 第 1 章 [08 生命周期](../Chapter-01-Foundations/08-lifetimes.md) 衔接。

Book → [10.1 泛型](../../00-Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md) · ER → [Item 02 · 闭包与 trait bound](../../01-ER/Chapter-01-Types/Item-02-express-common-behavior/README.md)
