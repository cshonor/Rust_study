# 2.4 Trait Bounds（Trait 限定）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

前置 → [05 编译与分发](./05-compilation-dispatch.md) · [07 孤儿规则](./07-coherence-orphan-rule.md) · 下一节 → [09 标记 Trait](./09-marker-traits.md)

---

Trait bound 把「类型必须具备的能力」写进函数签名，让编译器在**编译期**完成类型校验，并触发**单态化**（为每种传入类型生成专属机器码）。

---

## 一、Trait Bound 核心作用

| 作用 | 说明 |
|------|------|
| **能力约束** | 只有实现了所需 trait 的类型才能调用 |
| **编译期检查** | 违反 bound → 编译失败，而非运行时才发现 |
| **单态化入口** | `T: Trait` / `impl Trait` 触发静态分发特化 |
| **API 文档化** | 签名即契约：调用方需要哪些行为 |

→ 分发原理详见 [05 编译与分发](./05-compilation-dispatch.md)

---

## 二、三种基础语法与分发模式

### 1. 泛型约束 `<T: Trait>`

```rust
use std::fmt::Debug;

fn f<T: Debug + Clone>(x: T) {
    let y = x.clone();
    println!("{y:?}");
}
```

| 项 | 说明 |
|----|------|
| 分发 | **静态分发** |
| 原理 | 编译期为每个不同的 `T` 生成独立函数副本（单态化） |
| 多约束 | `+` 连接多个 trait，须**同时**满足 |
| where 等价 | `fn f<T>(x: T) where T: Debug + Clone {}` |

**何时用**：需要**命名** `T`、多参数**同一** `T`、或返回类型也是 `T`。

---

### 2. 参数位置 `impl Trait`

```rust
use std::fmt::Display;

fn g(x: impl Display) {
    println!("{x}");
}
```

| 项 | 说明 |
|----|------|
| 本质 | 匿名泛型参数的语法糖，同样**静态分发** |
| 语义 | **存在类型**：调用方传入任意实现 `Display` 的类型 |
| 限制 1 | 函数体内**不能**把参数类型提取为具名 `T`（除非改签名） |
| 限制 2 | 多个 `impl Trait` 参数默认是**不同**泛型，除非手写 `T` 绑定 |

```rust
// ❌ 两个 impl Display 不一定是同一类型
fn bad(a: impl Display, b: impl Display) {}

// ✅ 强制同类型
fn good<T: Display>(a: T, b: T) {}
```

---

### 3. `&dyn Trait`（Trait 对象）

```rust
use std::error::Error;

fn h(e: &dyn Error) {
    eprintln!("err: {e}");
}
```

| 项 | 说明 |
|----|------|
| 分发 | **动态分发** |
| 原理 | 胖指针 = 数据指针 + **vtable**，运行时查表调用 |
| 开销 | 间接调用 + 通常无法跨调用内联 |
| 适用 | 集合存多种实现：`Vec<Box<dyn Error>>` |

→ DST / 宽指针：[04 DST](./04-dst-wide-pointers.md) · [05 §动态分发](./05-compilation-dispatch.md)

---

### 三者核心区别

| 写法 | 分发 | 运行时开销 | 能否混存多种类型 |
|------|------|------------|------------------|
| `<T: Trait>` | 静态 | 无 vtable | 否（每处调用点一种 `T`） |
| `impl Trait` | 静态 | 无 vtable | 否 |
| `&dyn Trait` / `Box<dyn Trait>` | 动态 | vtable 间接调用 | **是** |

**HFT 直觉**：热路径优先 `T: Trait` / `impl Trait`；仅在必须异构集合或减二进制体积时用 `dyn`。

---

## 三、高阶 Trait 限定 HRTB

### 1. 核心语法 `for<'a>`

语义：**对任意生命周期 `'a`，后面的约束都成立**。

```rust
F: for<'a> Fn(&'a str)
```

含义：`F` 是闭包/函数，可接收**任意生命周期**的 `&str`，不受单一固定 `'a` 绑定。

### 2. 为什么必须用 HRTB？

**过窄写法**（`'a` 在函数签名层被固定）：

```rust
// ❌ F 只能处理与外部同一个 'a 的 &str
fn takes_fn<'a, F>(f: F)
where
    F: Fn(&'a str),
{
    f("hello");
}
```

缺陷：`'a` 在调用 `takes_fn` 时就固定，闭包无法适配「调用点各自不同」的短生命周期引用。

**正确 HRTB**：

```rust
fn takes_fn<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let s1 = "short";
    f(s1);
    let owned = String::from("long");
    f(&owned);
}
```

优势：`'a` 在**每次调用 `f` 时**才推导，闭包兼容所有合法 `&str` — 回调 API 标准写法。

→ 理论背景：[Ch01 · 08 生命周期 §方差](../Chapter-01-Foundations/08-lifetimes.md) · Nomicon [03 型变](../../03-Rust_Nomicon/03_Lifetime_Variance/03-lifetimes.md)

### 3. 典型场景

1. 接收闭包/回调的通用 API（`Iterator::map` 等）
2. 处理任意生命周期引用的工具函数
3. 解析器、事件回调、部分异步抽象

---

## 四、实操示例

→ 可运行：[trait-bounds-demo](./trait-bounds-demo/)

```bash
cd 02-RFR/Chapter-02-Types/trait-bounds-demo
cargo run
```

### 静态分发（`impl Trait`）

```rust
use std::fmt::Display;

fn print(x: impl Display) {
    println!("{x}");
}

fn main() {
    print(123);
    print("hello");
}
```

### 动态分发（`dyn Trait`）

```rust
use std::error::Error;

fn log_err(e: &dyn Error) {
    eprintln!("err: {e}");
}
```

### HRTB 完整示例

```rust
fn takes_any_str_closure<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("short lifetime");
    let owned = String::from("long string");
    f(&owned);
}

fn main() {
    takes_any_str_closure(|s| println!("got: {s}"));
}
```

去掉 `for<'a>` 后，与「固定外部 `'a`」的窄约束等价的写法往往会**编译失败**。

---

## 五、避坑清单

### 静态 vs 动态

| 坑 | 纠正 |
|----|------|
| 到处用 `Box<dyn Trait>`「更灵活」 | 热路径优先泛型；`dyn` 有 vtable + 难 inline |
| 以为 `impl Trait` 能混类型集合 | 不能；要异构集合用 `dyn` 或枚举 |
| 返回 `impl Trait` 与参数 `impl Trait` 混淆 | 返回位置是**存在类型**出口；见 [10 存在类型](./10-existential-types.md) |

### `impl Trait` 参数

| 坑 | 纠正 |
|----|------|
| `fn f(a: impl T, b: impl T)` 当同一类型 | 编译器视为**两个**匿名泛型 → 改 `fn f<T: T>(a: T, b: T)` |
| 在函数里需要 `T: Default` 等额外 bound | 改用显式 `<T>` + where |

### HRTB

| 坑 | 纠正 |
|----|------|
| `F: Fn(&'a str)` + 函数级 `'a` | 过窄；改 `for<'a> Fn(&'a str)` |
| 与生命周期省略规则打架 | 回调签名优先查是否该 HRTB |
| 以为 HRTB 是运行时概念 | **纯编译期**；影响类型是否可赋值 |

### Trait bound 其它

| 坑 | 纠正 |
|----|------|
| bound 写太多导致编译慢/二进制大 | 拆函数、用 `dyn` 减单态化、或关联类型收敛 |
| `T: 'static` 乱加 | 会拒绝借用的非 `'static` 数据；只在确实需要时加 |

---

## 六、速记

1. **`T: Trait` / `impl Trait` → 静态单态化；`dyn Trait` → vtable 动态。**  
2. **多参数要同一类型 → 显式 `<T>`，不要双 `impl`。**  
3. **回调吃任意 `&str` → `for<'a> Fn(&'a str)`。**

---

## 对照阅读

- Book → [10.1 泛型](../../00-Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md) · [10.2 trait](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md) · [10.2.3 impl Trait](../../00-Book/10-generics-traits-lifetimes/10.2.3-impl-Trait全解.md)
- RFR → [Ch01 · 08 生命周期](../Chapter-01-Foundations/08-lifetimes.md)
- ER → [Item 02 闭包与 trait bound](../../01-ER/Chapter-01-Types/Item-02-express-common-behavior/README.md) · [Item 12 泛型 vs trait 对象](../../01-ER/Chapter-01-Types/Item-12-generics-vs-trait-objects/README.md)
