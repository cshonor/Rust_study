# 3. Existential Types（存在类型）

> 所属：**Existential Types** · [← 章索引](./README.md)

前置 → [09 标记 Trait](./09-marker-traits.md) · [05 编译与分发](./05-compilation-dispatch.md) · [04 DST 与宽指针](./04-dst-wide-pointers.md) · 下一节 → [11 小结](./11-summary.md)

---

Existential Types（存在类型）在 Rust 里就是 **`impl Trait`**。名字来自数理逻辑的**存在量词 ∃**，语义：

> **存在**某一个具体类型 `T`，满足 `T` 实现了指定 trait，但对外**隐藏**这个 `T` 的真实名称。

核心两点：

1. 编译器**全程知道底层真实类型**，编译期大小确定，走**静态分发、单态化**，无运行时开销；
2. 对外只暴露 trait 接口，隐藏内部复杂类型，**稳定 API**、简化签名。

---

## 一、逻辑对应关系

| Rust 语法 | 逻辑量词 | 含义 |
|-----------|----------|------|
| `impl Trait` | **∃** `T`. `T: Trait` | 存在某类型满足约束（编译期唯一确定） |
| HRTB `for<'a>` | **∀** `'a` | 对**所有**生命周期 `'a` 约束都成立 |

→ HRTB 详解：[08 Trait 限定](./08-trait-bounds.md) §高阶 trait bound

---

## 二、两大使用位置

### 1. 返回值位置（最常用）

```rust
fn iter() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter().map(|x| x + 1)
}
```

`map` / `filter` / `flat_map` 链式组合后，真实类型名可长达数十行；`impl Iterator` 对外只承诺「返回迭代器」。

`async fn` 的返回值本质也是 **`impl Future`**，自动隐藏编译器生成的匿名 `Future` 状态机：

```rust
async fn fetch() -> u32 { 42 }
// 等价于返回 impl Future<Output = u32>
```

→ Async：[RFR 第 8 章](../Chapter-08-Asynchronous-Programming/README.md)

### 2. 函数参数位置

```rust
fn print(x: impl Display) {
    println!("{}", x);
}
```

语法糖，等价于匿名泛型：

```rust
fn print<T: Display>(x: T) {
    println!("{}", x);
}
```

同样**静态分发**；差别是无法在签名里给 `T` 命名，也不能在多处复用同一泛型名。

---

## 三、`impl Trait` vs `dyn Trait`

| 对比维度 | `impl Trait`（存在类型） | `dyn Trait`（trait 对象） |
|----------|--------------------------|---------------------------|
| 分发机制 | 静态分发，编译单态化 | 动态分发，运行时查 vtable |
| 内存大小 | 编译期确定（`Sized`） | DST，须 `Box` / `&` 等宽指针 |
| 异构集合 | 不支持；元素须同一底层类型 | 支持 `Vec<Box<dyn Trait>>` |
| 类型可见性 | 编译器知真实类型，调用方不可见 | 擦除具体类型，仅保留 vtable |
| 运行开销 | 无额外开销 | 微小间接调用开销 |

→ 分发模型：[05 编译与分发](./05-compilation-dispatch.md) · DST 载体：[04 DST 与宽指针](./04-dst-wide-pointers.md)

---

## 四、典型使用场景

| 场景 | 为什么用 `impl Trait` |
|------|------------------------|
| **长迭代器 / 闭包链** | 匿名闭包拼接类型名不可维护 |
| **`async fn`** | 匿名 `Future` 结构体无法手写 |
| **库 API 封装** | 内部换实现不破坏对外签名 |
| **简化泛型** | 单参数、无复用 `T` 时少写 `<T: Bound>` |

```rust
// 库内部可换实现，签名稳定
pub fn load_config() -> impl Read {
    std::fs::File::open("app.toml").unwrap()
}
```

---

## 五、核心限制

### 1. 不能存异构 `impl Trait`

```rust
// ❌ 不能 Vec<impl Display>
// ✅ 要多种类型共存 → dyn Trait
let items: Vec<Box<dyn Display>> = vec![
    Box::new(1),
    Box::new("hi"),
];
```

### 2. 返回位置只能是一种底层类型

```rust
fn bad(b: bool) -> impl Iterator<Item = i32> {
    if b {
        vec![1, 2].into_iter()   // 类型 A
    } else {
        (0..10)                  // 类型 B — ❌ 编译失败
    }
}
```

每个 `return` 路径的**具体类型必须相同**；分支返回不同迭代器 → 用 `enum` 包装或改 `Box<dyn Iterator<Item = i32>>`。

### 3. 参数位置每个 `impl` 是独立泛型

```rust
fn pair(a: impl Display, b: impl Display) {
    // a、b 可以是不同类型（如 i32 和 &str）
}

fn same<T: Display>(a: T, b: T) {
    // 需要同一类型时必须命名 T
}
```

---

## 六、选型一句话

| 需求 | 选型 |
|------|------|
| 隐藏复杂返回类型、零开销 | **`impl Trait`** |
| `Vec` 里混多种实现 | **`dyn Trait` + 间接** |
| 两个参数必须同类型 | **命名泛型 `<T>`** |
| 热路径、可内联 | 优先 **`impl Trait` / 泛型** |

---

## 七、易混区分

| 易混 | 纠正 |
|------|------|
| `impl Trait` = 运行时多态 | **静态分发**；编译器知道具体 `T` |
| `impl Trait` 可放 `Vec` 元素类型 | 不行；集合元素类型须在签名层固定 |
| 两个 `impl Display` = 同类型 | **独立**泛型参数，默认可不同 |
| `async fn` 返回 `dyn Future` | 默认 **`impl Future`**，非 trait 对象 |
| `∃` vs `∀` 可互换 | `impl Trait` 存在量化；`for<'a>` 全称量化，对偶关系 |

---

## 八、速记

1. **`impl Trait` = ∃T：存在某类型，对外匿名，编译期定死。**  
2. **返回位藏长类型；参数位 = 匿名 `<T: Trait>`。**  
3. **要异构集合 → `dyn`；要同参同型 → 命名 `T`。**

---

## 对照阅读

- Book → [10.2.3 impl Trait 全解](../../00-Book/10-generics-traits-lifetimes/10.2.3-impl-Trait全解.md) · [10.2 trait](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md)
- ER → [Item 12 泛型 vs trait 对象](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md)
- RFR → [05 编译与分发](./05-compilation-dispatch.md) · [08 Trait 限定 / HRTB](./08-trait-bounds.md)

→ 下一节：[11 小结](./11-summary.md)
