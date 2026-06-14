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

### 3. 单态化深度：运行时、体积、`#[inline]`、编译器

**运行时零开销**：同一泛型函数用 `i32` 和 `f64` 各调一次 → 编译器生成 **两个独立机器码版本**；调用处类型在编译期已知，**没有**运行时类型判断、**没有** vtable 间接跳转（与 `dyn` 对比 → 上表）。

```rust
fn add_one<T: Copy + std::ops::Add<Output = T> + From<u8>>(x: T) -> T {
    x + 1.into()
}
// add_one(1i32) 与 add_one(1.0f64) → 两份特化；各自直接 call / 可 inline
```

| 维度 | 单态化 |
|------|--------|
| 调用路径 | 编译期绑定具体符号 |
| 与 `dyn Trait` | 无 vtable；可跨 crate **devirtualize + inline** |
| 代价 | **编译时间** ↑、**二进制**可能重复膨胀 |

#### `#[inline]` 与 call 开销

`#[inline]` / `#[inline(always)]` 是**建议** LLVM 把函数体**嵌入调用处**，减少 `call` + 栈帧；热路径小函数常用。是否真 inline 仍由优化器决定 — 与单态化配合时，**具体类型**常让优化更容易成功。

跨 crate 默认不 inline 除非 `#[inline]` 或 LTO；HFT 可配合 **`lto = "thin"` / `"fat"`** 进一步合并。

#### 控制体积：类型参数组合爆炸

泛型参数或 trait bound 组合过多时，**每个 `(T, U, …)` 组合**都可能复制一份代码 — 二进制明显变大。常见缓解：

| 手段 | 思路 |
|------|------|
| **热路径 monomorph，边界 `dyn`** | 核心 loop 用具体 `T`；插件/IO 层用 trait object |
| **enum 静态多态** | 变体有限时用 `match` 代替无限 `T` → 本节 HFT 段 |
| **少泛型层嵌套** | `Iterator` 链、`async` 状态机叠加会乘性膨胀 |
| **查体积** | `cargo bloat --release`、`-C link-arg=-Wl,--print-memory-usage` |
| **LTO / strip** | 链接期删未用单态化实例（仍不如从源头少实例） |

#### 编译器如何收集单态化实例（现状 + 演进）

**当前 stable 流程**（[rustc dev guide · monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)）：

1. **Collector** 从 crate 根（`main`、导出项等）出发；
2. **遍历 MIR**，看到 `T = Concrete` 的调用就加入待 codegen 列表；
3. **递归展开**直到闭包 — demand-driven，「用到了才生成」。

**Flow-directed monomorphization**（[Pre-RFC · Rust Internals](https://internals.rust-lang.org/t/pre-rfc-flow-directed-monomorphization-collection-in-mir/24366)）— **编译器演进方向**，尚未替代上述 collector 成为默认路径：

- 在 **MIR 层**建 **类型流图**，收集约束 `τ ⊑ α`；
- **有限解** → 可安全单态化的实例集合；
- **循环类型流**（需无限特化，如 `T → Option<T> → T…`）→ **编译期结构性报错**，而不是递归爆栈或默默膨胀。

与 `impl Trait` / `async fn` / 高阶类型流相关的「会不会无限 monomorph」问题，正是这类分析要解决的。

---

## 二、单态化与内存：不同 `T` = 不同类型

**核心结论**：不同泛型实参在编译后是**完全独立的类型、独立内存布局、独立机器码** — **不会共用同一份内存模板**。

### 1. 泛型枚举会被单态化

Rust **没有** C++ 式「统一类模板实例」的运行时概念；编译器为**每一组不同的泛型实参**生成一份具体类型：

| 源码里写的 | 编译后 |
|------------|--------|
| `Option<u32>` | 类型 A：载荷 4B → 一套 tag+union 布局 |
| `Option<u64>` | 类型 B：载荷 8B → **另一套**布局 |

二者在类型系统里**互不相干** — `Option<u32>` 与 `Option<u64>` 不能互相赋值，内存尺寸、对齐、niche 规则都可能不同。

### 2. 对应「标签 + 联合体」怎么变

枚举底层模型 → [03 复合类型 · 枚举](./03-complex-types.md)

```text
Option<u32>                    Option<u64>
┌──────┬──────────┐           ┌──────┬──────────┐
│ tag  │ union T  │           │ tag  │ union T  │
│      │  u32 4B  │           │      │  u64 8B  │
└──────┴──────────┘           └──────┴──────────┘
size_of 通常 8B                size_of 通常 16B（x86_64 实测见 layout-demo）
```

- 联合体大小 = **该次单态化里 `T` 的大小**（再 + tag / padding / niche）
- `u32` 与 `u64` 触发**两次**独立 layout 计算，**不共享**载荷区模板

### 3. 跨文件（`a.rs` / `b.rs`）不影响规则

同一 **crate** 内，文件边界只是模块组织；编译器**遍历整个 crate**：

1. `a.rs` 用到 `Option<u32>` → 单态化，生成布局 + 代码
2. `b.rs` 用到 `Option<u64>` → 再单态化，**另一套**
3. 最终二进制里两份类型、两份 layout、相关函数各用各的格式

成千上万个文件用不同 `T`，就会生成对应数量的版本。

### 4. 两个细节

| 情况 | 行为 |
|------|------|
| **相同 `T` 跨文件** | 全 crate **只生成一份** `Option<u32>` — 布局与代码**复用** |
| **`T` 是 DST**（`[u8]`、`str`、`dyn Trait`） | 不能直接作枚举载荷；须 `Box<T>` / `&T` 等 **Sized 包装** — 单态化逻辑不变，载荷变成指针 |

```rust
// ❌ enum Bad<T> { Some(T) }  // T = [u8] 不行
// ✅ Option<Box<[u8]>>         // 载荷是指针，Sized
```

### 极简对照

| | |
|---|---|
| `Option<u32>` vs `Option<u64>` | **两套**独立枚举 + **两套** layout |
| 跨文件不同 `T` | **不**共用内存结构 |
| 跨文件相同 `T` | **共用**一份 layout 与代码 |

→ 实测：`Option<u32>` / `Option<u64>` → [`layout-demo`](./layout-demo/)

---

## 三、对象安全 (Object Safety)

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

## 四、选型速记（实战）

| 场景 | 推荐 | 说明 |
|------|------|------|
| 泛型参数 `T: Trait` | **静态** | 编译期单态化，性能最优 |
| 异构集合、插件、运行时换实现 | **`dyn Trait`** | `Vec<Box<dyn Handler>>` — 灵活，有开销 |
| 返回闭包 / 长迭代器链 | **`impl Trait`** | 隐藏具体类型，**仍静态** → [10](./10-existential-types.md) |
| 多种实现但热路径要快 | **enum + match** | 静态分发模拟多态（见下） |

---

## 五、HFT 场景要点

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

### 5. 二进制体积：泛型组合过多时

策略层若对 **几十种 `T: Strategy`** 全 monomorph，release 二进制可从 MB 级涨到 **数十 MB**（视依赖与 LTO 而定）。HFT 常见做法：

- **订单簿 / 撮合**：1～3 个具体类型 + 泛型，或 **enum Strategy**；
- **序列化 / 日志 / 配置**：边界用 `dyn` 或具体类型，**不**把泛型传进 inner loop；
- 发版前跑 **`cargo bloat --release -n 20`** 看哪几个单态化符号占体积。

FFI 边界通常 **不用 `dyn`** → [第 11 章 FFI](../Chapter-11-Foreign-Function-Interfaces/README.md)

---

## 六、与汇编 / 优化的直觉（不背指令，记现象）

| | 静态 | `dyn` |
|---|------|-------|
| 典型 call | 直接 `call` 已知符号，或 **inline 消失** | `call [vtable+offset]` 类间接 |
| 优化器 | 常能 **devirtualize**（若类型可证唯一） | 难 inline 跨 vtable |
| 排查 | `cargo asm` / 04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17 | 对比同逻辑泛型版 |

→ [04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17 · ch04 分发 IR 对照](../../04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/part02_src_to_machine/chapter04_ir_basic/notes/04_dispatch_static_vs_dyn.md) · [04_dispatch_O0.ll](../../04_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/ir_samples/optimize_compare/04_dispatch_O0.ll)

---

## 易错点

| 易错 | 纠正 |
|------|------|
| 「要 polymorphism 就必须 dyn」 | **enum / 泛型** 也是多态，且常更快 |
| `impl Trait` = 动态分发 | **静态** — 只是隐藏类型名 |
| 所有 trait 都能 `Box<dyn T>` | 须 **object-safe** |
| HFT 里到处 `dyn` 没问题 | 热路径 **优先 monomorph / enum** |
| `Option<u32>` 和 `Option<u64>` 共用 layout | **不同 T → 不同单态化 → 不同 layout** |
| `#[inline]` 一定 inline | **提示**；跨 crate 还需 LTO 等配合 |
| flow-directed monomorph 已默认启用 | **Pre-RFC 方向**；stable 仍 mainly MIR collector |

---

## 对照阅读

- ER → [Item 12 · 泛型 vs trait object](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) · [06 dispatch 入门](../../01-ER/Chapter-02-Traits/Item-12-generics-vs-trait-objects/06-dispatch-beginner-guide.md)
- Book → [17.2 trait 对象](../../00-Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) · [10.2.3 impl Trait](../../00-Book/10-generics-traits-lifetimes/10.2.3-impl-Trait全解.md)
- 下一节 → [06 泛型 Trait](./06-generic-traits.md)
