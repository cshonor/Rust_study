# Item 12: Understand generics vs trait objects trade-offs

> **Effective Rust** · [Chapter 2 — Traits](../ER-本书目录.md)  
> **中文**：理解泛型与 trait 对象之间的权衡  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo / 代码练习

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 泛型、trait bound | [10.1 泛型](../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[10.2 trait](../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| `dyn Trait`、trait 对象 | [17.2 trait 对象](../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| 胖指针 | [Item 8](../Chapter-01-Types/Item-08-references-pointers.md) |
| 单态化、高级 trait | [19.2 高级 trait](../../Book/19-advanced-features/19.2-高级trait.md) |
| 闭包 vs `fn` 静/动分发 | [Item 2](../Chapter-01-Types/Item-02-express-common-behavior.md) |

---

## 1. 核心知识点与关键定义

### 泛型 + Trait Bound

```rust
fn draw<T: Draw>(x: &T) { ... }
// 或 where T: Debug + Draw
```

- 编译期 **单态化（Monomorphization）**：每种具体 `T` 一份机器码 → **静态分发**。
- 代价：编译更慢、二进制可能**膨胀**；收益：可内联、无 vtable 间接跳转。

### Trait 对象 `dyn Trait`

```rust
let shapes: Vec<&dyn Draw> = vec![&circle, &square];
```

- **胖指针**：数据指针 + **vtable** 指针 → **动态分发**。
- 形式：`&dyn Trait`、`Box<dyn Trait>` 等。

### 对象安全（Object Safety）

Trait 要能做成 `dyn Trait`，通常需满足（简化记忆）：

1. 方法不能是**泛型方法**（无 `<T>` 等方法级泛型）。
2. 除 `self` 接收者外，签名里不能出现 **`Self`**（运行期不知 `Self` 大小）。

→ 用 **`where Self: Sized`** 把「返回 `Self`」等方法**排除在 trait object 调用之外**，trait 仍可 `dyn`。

---

## 2. 逻辑脉络

```text
泛型：编译期特化 → 快、可组合多 bound → 代码体积↑
dyn：  运行期 vtable → 异构集合、省代码 → 间接调用开销
```

### 多重 bound vs 单一 vtable

- 泛型：`T: Debug + Draw` 在**编译期**同时要求两者，可写 `where` 解锁方法。
- `dyn`：一个 vtable 对应**一个** trait；多重 trait 组合在运行期会复杂（supertrait 融合 vtable，见下）。

### Supertrait ≠ OOP 继承

```rust
trait Shape: Draw { fn area(&self) -> f64; }
```

- 表示 **also-implements**（实现 `Shape` 必须也实现 `Draw`），不是「子类 is-a 父类」。
- `dyn Shape` 的 vtable 含 `Draw` 方法，但 **Upcasting** 历史上受限（见 §5、§6）。

---

## 3. 重点结论与实用要点

| 优先 | 场景 |
|------|------|
| **默认用泛型** | 性能、组合表达、编译期已知类型 |
| **用 `dyn Trait`** | 异构集合（`Vec<&dyn Shape>`）；严控二进制体积/编译时间；类型编译期未知（`dlopen` 等） |

### `where Self: Sized` 技巧

```rust
trait Stamp: Draw {
    fn make_copy(&self) -> Self where Self: Sized;
}
```

- `&dyn Stamp` 合法；
- **`stamp.make_copy()`** 在 trait object 上**不可调用**（仅具体类型上可调）。

---

## 4. 案例与代码要点

### 泛型多重 bound

```rust
fn show<T>(draw: &T)
where
    T: std::fmt::Debug + Draw,
{
    println!("{:?} has bounds {:?}", draw, draw.bounds());
}
```

只有同时实现 `Debug + Draw` 的类型才能调用。

### Trait 对象异构集合

```rust
fn render_all(items: &[&dyn Draw]) {
    for item in items {
        item.draw();
    }
}
```

### 对象安全 + `Sized` 方法

```rust
let square = Square::new(/* ... */);
let stamp: &dyn Stamp = &square;
// stamp.make_copy(); // ❌ trait object 上不可用
let copy = square.make_copy(); // ✅ 具体类型
```

---

## 5. 易错细节

### Upcasting 误区

- 不要以为 **`&dyn Shape` 能随意当 `&dyn Draw`** 传递（即使 `Shape: Draw`）。
- 旧版 Rust：`Shape` vtable 是融合体，**不能**在运行期「剥离」成纯 `Draw` vtable。
- **Rust 1.76+**：`trait_upcasting` 逐步支持部分向上转型 → 见 §6。

### 性能盲区

`dyn` 方法调用路径大致：

```text
胖指针 → 读 vtable → 取函数指针 → 间接跳转（难内联）
```

热点路径优先泛型；用 bench 验证。

---

## 6. 后续拓展（待补）

- [ ] **Rust 1.76+ `trait_upcasting`**：`dyn Shape` → `dyn Draw` 与 vtable 变化
- [ ] `cargo bench`：单态化 vs `dyn` 纳秒级对比

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 泛型 | 静态分发；快；可能膨胀 |
| `dyn` | vtable；异构集合；间接开销 |
| 对象安全 | 无泛型方法；无裸 `Self` |
| `Self: Sized` | 保留 dyn，方法仅具体类型可调 |
| 默认 | 先泛型，真要擦除再用 dyn |
