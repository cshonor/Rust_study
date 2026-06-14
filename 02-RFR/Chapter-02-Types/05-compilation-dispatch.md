# 2.1 Compilation and Dispatch（编译与分发）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

Trait 定义类型间的**可替换行为契约**；编译器如何把方法调用变成机器码，就是**分发模型**。

前置 → [04 DST 与宽指针](./04-dst-wide-pointers.md)（`dyn Trait` 载体）· 存在类型 → [10 impl Trait](./10-existential-types.md)

---

## 一、两种分发模型

### 1. 静态分发（Monomorphization / 单态化）

**原理**：编译器为每个具体类型 `T` **生成一份专属代码**。

```rust
fn process<T: Handler>(h: T) {
    h.on_tick();
}
// 对 HandlerA、HandlerB 各生成一份 process 特化
```

| 优点 | 缺点 |
|------|------|
| **无 vtable 间接调用**；易 **inline** | **编译时间** ↑、**二进制**可能膨胀（代码重复） |
| 峰值延迟低 — **HFT 热路径首选** | 无法直接做「多种具体类型混在同一集合」 |

**典型写法**：`fn foo<T: Trait>(x: T)` · `impl Trait for Concrete` · 泛型 struct 字段

---

### 2. 动态分发（`dyn Trait`）

**原理**：运行时经 **vtable** 查方法地址 — 一次**间接调用**。

```rust
fn process(h: &dyn Handler) {
    h.on_tick(); // 经 vtable 跳转
}

let handlers: Vec<Box<dyn Handler>> = vec![/* 异构实现 */];
```

| 优点 | 缺点 |
|------|------|
| **一份** trait object 代码；二进制相对省 | **间接调用**；难 inline；分支预测更差 |
| **异构集合**、运行时选实现 | 依赖 **DST + 宽指针** → [04](./04-dst-wide-pointers.md) |

**典型写法**：`&dyn Trait` · `Box<dyn Trait>` · `Arc<dyn Trait>`

---

### 对照表（必背）

| | **静态** | **动态 `dyn`** |
|---|----------|----------------|
| 解析时机 | **编译期** | **运行期**（vtable） |
| 调用 | 直接 / 可 inline | 间接跳转 |
| 异构 `Vec` | 需 **enum** 等 | `Vec<Box<dyn T>>` 自然 |
| 代码体积 | 多份 monomorph | 少份 + vtable |
| HFT 热路径 | ✅ **优先** | ⚠️ 慎用 |

---

## 二、对象安全 (Object Safety)

**并非所有 trait 都能 `dyn Trait`** — vtable 必须为**固定布局**的方法表。

### 常见「阻碍 `dyn`」的情况

| 问题 | 例子 | 为何不行 |
|------|------|----------|
| **泛型方法** | `fn bar<T>(&self, x: T)` | 无法为所有 `T` 列 vtable 条目 |
| **返回 `Self`（Sized）** | `fn new() -> Self` | trait object 上 `Self` 大小未知 |
| **要求 `Self: Sized` 的方法** | `fn clone(&self) -> Self where Self: Sized` | `dyn Trait` 是 **DST**，非 Sized |
| **关联类型无约束到 object-safe 用法** | 视具体定义 | 见 Reference / Clippy `object_safe` |

```rust
// ❌ 通常不能 dyn
trait Bad {
    fn generic<T>(&self, x: T);
    fn make() -> Self;
}

// ✅ 常见 object-safe 形态
trait Good {
    fn handle(&self, msg: &str);
    fn handle_mut(&mut self, msg: &str);
}
```

→ 设计 API 时先定：**要不要 trait object？** → [第 3 章 Designing Interfaces](../Chapter-03-Designing-Interfaces/README.md)

---

## 三、选型速记（实战）

| 场景 | 推荐 | 说明 |
|------|------|------|
| 泛型参数 `T: Trait` | **静态** | 编译期单态化，性能最优 |
| 异构集合、插件、运行时换实现 | **`dyn Trait`** | `Vec<Box<dyn Handler>>` — 灵活，有开销 |
| 返回闭包 / 长迭代器链 | **`impl Trait`** | 隐藏具体类型，**仍静态** → [10](./10-existential-types.md) |
| 多种实现但热路径要快 | **enum + match** | 静态分发模拟多态（见下） |

---

## 四、HFT 场景要点

### 1. 热路径：静态分发优先

交易、行情、订单簿更新等核心 loop：

- 用 **`T: Strategy`** 泛型 monomorph，或 **具体类型**
- 避免 `&dyn Strategy` 在 inner loop 里反复 vtable 调用

### 2. 用 enum 做「静态多态」

不需要 runtime 插件时，用 **enum 包所有实现** + **`match`** — 编译器可优化为跳转表或直接 inline：

```rust
enum Handler {
    A(HandlerA),
    B(HandlerB),
}

impl Handler {
    fn on_tick(&mut self) {
        match self {
            Handler::A(h) => h.on_tick(),
            Handler::B(h) => h.on_tick(),
        }
    }
}
```

第三方如 **`enum_dispatch`** 可减样板 — 本质仍是静态分发。

### 3. `impl Trait` 作返回类型

复杂 iterator / 闭包链：**不引入 `dyn`**，又不必暴露具体类型名：

```rust
fn ticks() -> impl Iterator<Item = u64> { /* ... */ }
```

### 4. 何时才用 `dyn Trait`

- 插件系统、**运行时**加载模块
- 配置驱动的异构 handler 列表，且**不在**纳秒级 inner loop
- 接受 vtable + 宽指针成本，换表达力

FFI 边界通常 **不用 `dyn`** → [第 11 章 FFI](../Chapter-11-Foreign-Function-Interfaces/README.md)

---

## 五、与汇编 / 优化的直觉（不背指令，记现象）

| | 静态 | `dyn` |
|---|------|-------|
| 典型 call | 直接 `call` 已知符号，或 **inline 消失** | `call [vtable+offset]` 类间接 |
| 优化器 | 常能 **devirtualize**（若类型可证唯一） | 难 inline 跨 vtable |
| 排查 | `cargo asm` / llvm_insight | 对比同逻辑泛型版 |

→ [llvm_insight](../../llvm_insight/part02_src_to_machine/chapter04_ir_basic/README.md)

---

## 易错点

| 易错 | 纠正 |
|------|------|
| 「要 polymorphism 就必须 dyn」 | **enum / 泛型** 也是多态，且常更快 |
| `impl Trait` = 动态分发 | **静态** — 只是隐藏类型名 |
| 所有 trait 都能 `Box<dyn T>` | 须 **object-safe** |
| HFT 里到处 `dyn` 没问题 | 热路径 **优先 monomorph / enum** |

---

## 对照阅读

- ER → [Item 12 · 泛型 vs trait object](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) · [06 dispatch 入门](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/06-dispatch-beginner-guide.md)
- Book → [17.2 trait 对象](../../00-Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) · [10.2.3 impl Trait](../../00-Book/10-generics-traits-lifetimes/10.2.3-impl-Trait全解.md)
- 下一节 → [06 泛型 Trait](./06-generic-trait.md)
