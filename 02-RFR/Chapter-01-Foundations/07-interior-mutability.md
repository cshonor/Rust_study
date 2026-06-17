# 3.3 Interior Mutability（内部可变性）

> 所属：**Borrowing and Lifetimes** · [← 章索引](./README.md)

前置 → [05 共享引用](./05-shared-references.md) · [06 可变引用](./06-mutable-references.md) · 下一节 → [08 生命周期](./08-lifetimes.md)

---

## 零、前置：外部可变性（理解内部可变性的前提）

### 1. 什么是外部可变性

`let` / `let mut` 控制的是**绑定层**能不能改 — 由编译器**编译期**静态检查：

| 声明 | 绑定 | 通过该绑定拿 `&mut` |
|------|------|---------------------|
| `let a = 10` | 不可重新赋值 `a = 20` | ❌ 不能 `&mut a` |
| `let mut a = 10` | 可重新赋值 | ✅ 可 `&mut a`（仍受借用规则约束） |

### 2. 借用铁律（复习）

同一时刻**二选一**：

1. 任意多个 `&T`（共享读）  
2. **至多一个** `&mut T`（独占写）

有活跃 `&T` → 编译器禁止一切修改。零运行开销，冲突直接**编译失败**。

→ 详述 [05](./05-shared-references.md) · [06](./06-mutable-references.md)

### 3. 外部可变性的短板（为何需要内部可变）

| 痛点 | 例子 |
|------|------|
| API 只能 `&self` | trait 方法签名固定，内部却要改计数器 |
| `Rc` 只有 `&T` | 多所有者共享，静态规则拿不到 `&mut` |
| `mut` 粒度太粗 | 整结构体 `mut`，只想改缓存/计数字段 |

**内部可变性** = 在这些场景下，仍保持外层 `let` / `&self`，把「能不能改」交给**容器内部**。

---

## 一、内部可变性 · 完整定义

### 核心定义

**可变性不再由外部的 `mut` 单独决定，而是封装在容器里**：外层常用普通 `let`（绑定/指针关系不变），通过容器专属 API 修改**内部**数据。

**大白话**：**外面的绑定不动，只改盒子里的东西**。

### 逻辑对比

| | **外部可变性** | **内部可变性** |
|---|----------------|----------------|
| 谁管权限 | 变量有没有 `mut` | 容器内部（运行时/锁） |
| 检查时机 | **编译期** | **运行时**（`RefCell` panic）或锁 |
| 表层借用 | `&mut T` 或 `mut` 绑定 | 常是 `&RefCell<T>`、`&self` |

### 容器分类

| 场景 | 容器 |
|------|------|
| 单线程 · `Copy` 小值 | `Cell<T>` |
| 单线程 · 通用 | **`RefCell<T>`**（本节重点） |
| 多线程 | `Mutex<T>` · `RwLock<T>` |

---

## 二、底层基石：`UnsafeCell<T>`

所有内部可变容器的唯一底层原语 — **opt-out** 普通 `&T` 的「永不修改」优化假设：

```text
普通 &T     → LLVM 可假定只读（load 消除、寄存器缓存）
UnsafeCell  → 声明「共享路径下也可能写」，禁止错误优化
```

用户不直接使用 `UnsafeCell`；`RefCell` / `Cell` / `Mutex` 封装并承担安全契约。

→ Nomicon [五种 unsafe 能力](../../03-Rust_Nomicon/01_Safe_Unsafe/03-five-powers.md) · [05 §LLVM](./05-shared-references.md)

---

## 三、`RefCell<T>` 深度拆解

### 1. 逻辑结构（概念）

```text
RefCell<T> {
    borrow_flag:  运行时借用状态（读计数 / 是否在写）
    value:        UnsafeCell<T>   // 真实业务数据
}
```

**自带读写门禁计数器的盒子** — 编译器不再对 `T` 做静态借用检查，由 `borrow_flag` 在运行时模拟规则。

### 2. 内存与 `let` 约束（指针对齐你的思路）

```rust
let x = RefCell::new(10);
```

| 层级 | 典型布局 | `let x` 约束什么 |
|------|----------|------------------|
| **栈上 `x`** | `RefCell<i32>` 整盒常在栈上 | **绑定不可变**：不能 `x = RefCell::new(99)` 换一整盒 |
| **盒内 `T`** | `UnsafeCell` 里的 `10` | 不靠外部 `mut`；用 `borrow` / `borrow_mut` 改 |

```rust
let x = Rc::new(RefCell::new(10));
```

| 层级 | 布局 |
|------|------|
| 栈上 | `Rc` 指针（`let` 锁的是指针绑定） |
| 堆上 | `RefCell` + 内部 `T`（`Rc` 分配） |

**关键**：`let` 锁的是 **「`x` 这个绑定/指针不能换指向」**，不是锁死盒内数字；盒内修改走 `borrow_mut()`。

> 纠正常见说法：`RefCell::new(10)` **默认在栈上**，不是必然堆分配；**堆**出现在 `Rc<RefCell<T>>` / `Box<RefCell<T>>` 等组合里。

### 3. 三个核心 API · 运行时规则

| 方法 | 作用 | 规则 |
|------|------|------|
| `.borrow()` | `Ref<T>` 共享读 | 无活跃写时可调用；**多个读可共存** |
| `.borrow_mut()` | `RefMut<T>` 独占写 | **读计数=0 且无写** 才可；独占 |
| 违规 | — | 不编译报错 → **运行 panic**（已有 `Ref` 时再 `borrow_mut` 等） |

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10);

    let r1 = x.borrow();
    let r2 = x.borrow();        // ✅ 多读共存
    println!("{}", r1);

  // let w = x.borrow_mut();    // ❌ 运行 panic：仍有活跃 Ref

    drop(r1);
    drop(r2);

    let mut w = x.borrow_mut(); // ✅ 无活跃借用
    *w = 20;
    println!("{}", x.borrow());  // 20
}
```

**对比**：外部 `mut` → **编译**报错；`RefCell` 违规 → **运行** panic。

### 4. C++ 思维对齐（`int* const`）

```cpp
int* const box = new int(10);
// box = 别的地址;   // ❌ 指针顶层 const（≈ Rust let x）
*box = 20;             // ✅ 改指向内容（≈ borrow_mut）
```

差异：C++ 无运行时读写互斥 → 数据竞争 = UB；`RefCell` 用计数器，违规 **panic**。

---

## 四、标准库容器速查

| 容器 | 场景 | 特性 | 违规 |
|------|------|------|------|
| `Cell<T>` | 单线程 · `Copy` | `get`/`set`，**无**内部引用 | 类型约束保证安全 |
| `RefCell<T>` | 单线程 · 通用 | 运行时 borrow 计数 | **panic** |
| `Mutex<T>` / `RwLock<T>` | 多线程 | 锁同步 | 死锁 / **中毒** |

→ [RFR 第 10 章 并发](../Chapter-10-Concurrency-and-Parallelism/README.md) · Nomicon [Send/Sync](../../03-Rust_Nomicon/07_Concurrency_Atomic/02-send-sync.md)

---

## 五、三大核心场景

### 场景 1 · `&self` 签名，内部改状态

```rust
use std::cell::RefCell;

struct Counter {
    count: RefCell<u32>,
}

impl Counter {
    fn add(&self) {                    // 不能改成 &mut self
        *self.count.borrow_mut() += 1;
    }
}
```

### 场景 2 · `Rc<RefCell<T>>` 多所有者共享修改

```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(100));
let p1 = Rc::clone(&data);
let p2 = Rc::clone(&data);

*p1.borrow_mut() = 200;
println!("{}", p2.borrow());   // 200
```

`Rc` 只给 `&T` → 内部可变性是单线程共享写的经典组合。

### 场景 3 · 细粒度：整体不可变，局部字段可变

```rust
struct Cache {
    key: String,              // 核心字段：无 mut 即不可换绑
    hits: RefCell<u32>,       // 仅统计可运行时改
}
```

外部 `let cache` 不必 `mut`；误改 `key` 仍被编译器拦住，只有 `hits` 走 `borrow_mut`。

---

## 六、更多代码示例

### `Cell` — 无内部引用

```rust
use std::cell::Cell;

let data = Cell::new(0);
let shared = &data;
shared.set(10);
println!("{}", shared.get());
```

### 多句柄同一 `RefCell`

```rust
let data = RefCell::new(5);
let r1 = &data;
let r2 = &data;

{
    let _a = r1.borrow_mut();
    // let _b = r2.borrow_mut(); // ❌ panic
}
let _c = r2.borrow(); // ✅ 上一段 mut 已结束
```

### `RefCell` + 非 `Copy` 类型

```rust
let cell = RefCell::new(vec![1, 2, 3]);
let mut w = cell.borrow_mut();
w.push(4);
```

---

## 七、外部可变性 vs `RefCell` 完整对比

| 维度 | 外部（`let` / `let mut`） | `RefCell` 内部可变性 |
|------|---------------------------|----------------------|
| 权限位置 | 外部 `mut` | 容器内 `borrow_flag` |
| 检查 | **编译期** | **运行时** |
| 开销 | 零 | 计数增减（微小） |
| 读写规则 | 编译互斥 | 运行模拟：多读或一写 |
| 外层绑定 | `let` 不可换绑；`let mut` 可 | 外层通常 **`let`**，绑定不换 |
| 适用 | 简单路径、极致性能 | `&self`、`Rc`、细粒度局部改 |

| | `&mut T` | `RefCell` / `Mutex` |
|---|----------|-------------------|
| 校验 | 编译期 | 运行时 / 锁 |
| 表层 | 独占 `&mut` | 常是 `&Container` |
| 违反 | 编译失败 | panic / 死锁 |

---

## 八、常见误区

| 误区 | 纠正 |
|------|------|
| 「有 `&T` 就完全不能改」 | 直接改不行；**内部可变**是显式例外 |
| 「`RefCell` 绕过所有借用规则」 | 规则仍在，**推迟到运行时** |
| 「`RefCell::new` 一定在堆上」 | 单独 `let x = RefCell::new(10)` 常在**栈**；`Rc`/`Box` 才常见堆 |
| 「`let x` 锁死盒内数字」 | 锁的是**绑定**；盒内用 `borrow_mut` |
| 「`Cell` = 小 `RefCell`」 | `Cell` 仅 `Copy`、`get`/`set`；无 `borrow` |
| 「`Mutex` = 多线程 `RefCell`」 | 类似互斥，但还有 **Send/Sync**、中毒、阻塞 |

---

## 九、一句话总纲

1. **外部可变性**：编译器看外面有没有 `mut`，冲突**编译期**卡死。  
2. **内部可变性**：外层 `let` 绑定不动，数据放进带**门禁计数器**的盒子里，**运行时**管读写。  
3. **`RefCell`**：没打破「绑定不可换」，只是把「盒内修改权」从编译期交给容器，弥补静态规则死板，仍保内存安全（违规 panic，非 UB）。

---

## 十、下一节

→ [08 生命周期](./08-lifetimes.md)

---

## 对照阅读

- 前置 → [05 共享引用](./05-shared-references.md) · [06 可变引用](./06-mutable-references.md)
- 速记 → [05-08-borrowing-lifetimes-cheat-sheet.md](./05-08-borrowing-lifetimes-cheat-sheet.md)
- Book → [15.5 RefCell 与内部可变性](../../00-Book/15-smart-pointers/15.5-RefCell与内部可变性.md)
- ER → [Item 17 共享状态并行](../../01-ER/Chapter-03-Concepts/Item-17-shared-state-parallelism/README.md)
