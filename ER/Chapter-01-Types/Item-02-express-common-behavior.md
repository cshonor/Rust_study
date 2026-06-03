# Item 2: Use the type system to express common behavior

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：使用类型系统表达公共行为  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 函数、方法、`impl` | [3.3 函数](../../Book/03-common-concepts/3.3-函数.md)、[5.3 方法语法](../../Book/05-structs/5.3-方法语法.md) |
| 闭包、`Fn*` | [13.1 闭包](../../Book/13-iterators-closures/13.1-闭包.md)、[19.4 高级函数与闭包](../../Book/19-advanced-features/19.4-高级函数与闭包.md) |
| Trait、泛型 | [10.1 泛型](../../Book/10-generics-traits-lifetimes/10.1-泛型数据类型.md)、[10.2 trait](../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| Trait 对象 | [17.2 trait 对象](../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| 单态化 vs 动态分发 | [Item 12](../Chapter-02-Traits/Item-12-generics-vs-trait-objects.md)（ER） |

---

## 1. 核心知识点与关键定义

### 方法与函数

| 形式 | 说明 |
|------|------|
| **函数** | 自由函数，组织代码复用 |
| **方法** | 与类型绑定的函数，写在 `impl` 块中 |
| `&self` | 不可变借用，只读 |
| `&mut self` | 可变借用，可改实例 |
| `self` | 按值接收，**消耗**所有权 |

### 函数指针 `fn`

- 指向代码地址，**不捕获环境**。
- 实现 `Copy`、`Eq`，可当作普通值传递、比较（在显式转为 `fn` 类型之后）。

### 闭包（Closures）

- 可捕获上下文的匿名函数。
- 编译器为每个闭包生成**唯一的匿名 struct**，保存捕获的引用或移动进来的值。

### 闭包 Trait（由捕获方式决定）

| Trait | 捕获方式 | 调用 |
|-------|----------|------|
| **`FnOnce`** | 移动或消耗环境 | 通常只能调用一次 |
| **`FnMut`** | `&mut` 借用环境 | 可多次调用并修改环境 |
| **`Fn`** | `&` 借用环境 | 可多次只读调用 |

### Trait（特征）

- 一组**共享行为的契约**；类似 Go/Java 接口、C++ 纯虚类。
- **标记 Trait**：无方法的空 trait（如 `StableSort`），在类型层面编码「签不了名」的语义。

---

## 2. 逻辑脉络与知识点关联

```text
自由函数 → 方法（绑在类型上）
    → fn（无环境）
    → 闭包（带环境 + Fn*）
    → Trait（多态抽象）
         ├─ 泛型 + Trait Bound → 单态化 / 静态分发
         └─ dyn Trait → vtable / 动态分发
```

### 闭包 Trait 的「向下兼容」

- 需要 **`FnOnce`** 时：可传 `FnOnce` / `FnMut` / `Fn`。
- 需要 **`FnMut`** 时：可传 `FnMut` / `Fn`。
- 需要 **`Fn`** 时：只能传 `Fn`。

（参数位置越宽，调用方越灵活。）

### 两种 Trait 分发

| 机制 | 时机 | 代表写法 | 特点 |
|------|------|----------|------|
| **静态分发** | 编译期 | `fn f<T: Trait>(t: T)` | 单态化，零开销抽象，可能代码膨胀 |
| **动态分发** | 运行期 | `&dyn Trait` | vtable 胖指针，类型擦除，集合可同构 |

---

## 3. 重点结论与实用要点

1. **回调 API 优先 `Fn*`，少用裸 `fn`**——让调用方能捕获环境。
2. **参数用「能干活的最宽」`Fn*`**——只调用一次就写 `FnOnce`，别无谓收紧为 `Fn`。
3. **参数优先 Trait bound，少绑死具体 struct**——便于扩展与测试。
4. **无 Trait bound 的泛型 `T` 几乎啥也不能干**——只能 move/drop；要调方法必须 `T: SomeTrait`。

---

## 4. 案例与代码要点

### 函数名 ≠ 可直接比较的 `fn`

```rust
fn sum(x: i32, y: i32) -> i32 { x + y }

let op1 = sum;
let op2 = sum;
// assert!(op1 == op2); // ❌ 每个函数有唯一内部类型，未实现 Eq

let op: fn(i32, i32) -> i32 = sum;
let op2: fn(i32, i32) -> i32 = sum;
assert!(op == op2); // ✅ 显式函数指针类型后可比较
```

### 闭包底层 ≈ 带捕获字段的 struct

```rust
let amount_to_add = 3;
let add_n = |y| y + amount_to_add;
// 编译器生成类似：
// struct InternalContext<'a> { amount_to_add: &'a i32 }
// 并实现 call 逻辑
```

因此 **`add_n` 不能**传给只接受 `fn(u32) -> u32` 的参数（缺少捕获环境）。

### API 对比（示意）

```rust
// 较死板：调用方不能捕获环境
fn apply_fn(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }

// 更灵活：调用方可传闭包
fn apply<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
```

---

## 5. 易错细节

### C++ 模板 vs Rust 泛型

| | C++ 模板 | Rust 泛型 |
|--|----------|-----------|
| 约束 | 往往隐式（有同名方法就能用） | **必须显式 Trait bound** |
| 风险 | 「鸭子类型」误用可能很晚才暴露 | 契约在签名处拦截 |

### Trait 对象的对象安全（Object Safety）

并非所有 trait 都能写成 `dyn Trait` / `&dyn Trait`。常见**不能**对象化的情况：

- 方法返回 `Self`；
- 方法带未消去的泛型参数。

→ 编译器无法建稳定 vtable，**拒绝**生成 trait 对象。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 02](../ER-拓展索引.md#item-02)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| `self` 三种 | `&` 读、`&mut` 改、`self` 拿走 |
| `fn` vs 闭包 | `fn` 无捕获；闭包有唯一类型 + `Fn*` |
| `Fn*` 选宽 | 能 `FnOnce` 就别写死 `Fn` |
| 泛型 | 无 bound 的 `T` 几乎只能 move |
| `dyn Trait` | 要对象安全；否则用泛型 |
