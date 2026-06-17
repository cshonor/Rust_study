# 2.5 Marker Traits（标记 Trait）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

前置 → [08 Trait 限定](./08-trait-bounds.md)（[08.1](./08-1-syntax-static-dynamic.md)）· [04 DST 与宽指针](./04-dst-wide-pointers.md) · 下一节 → [10 存在类型](./10-existential-types.md)

---

标记 Trait（Marker Trait）是一类**不含方法 / 关联常量、仅承载语义标记**的特殊 trait。本身没有可供调用的函数，作用是向编译器传递类型的安全属性、内存布局属性，约束编译检查、指导优化。

典型代表：`Sized`、`Copy`、`Send`、`Sync`、`Unpin`。

---

## 一、与普通 Trait 的区别

| | **Marker Trait** | **普通 Trait** |
|---|------------------|----------------|
| 方法 | 无（或几乎无） | 有 `fn` 等多态行为 |
| 用途 | 编译器契约、类型检查 | API 行为抽象 |
| 运行时 | **零开销**（编译期完成） | 静态或动态分发 |
| `impl` 约束 | `Send`/`Sync` 等可 `unsafe impl` | 通常 safe `impl` |
| 孤儿规则 | **同样适用**（不能双外部 `unsafe impl Send`） | 同样适用 |

---

## 二、五大核心标记 Trait

### 1. `Sized`

| 项 | 说明 |
|----|------|
| **契约** | 编译期能确定**固定** `size_of` |
| **默认** | 泛型 `T` **隐含** `T: Sized` |
| **自动** | `i32`、`String`、`Vec`、普通 struct 等 |
| **例外** | DST：`[T]`、`str`、`dyn Trait` → **`!Sized`** |
| **语法** | `T: ?Sized` = 允许 Sized **或** DST |

```rust
// T 默认隐含 T: Sized；?Sized 放开限制
fn foo<T: ?Sized>(x: &T) {}

fn takes_slice(s: &str) {}      // str 是 !Sized，通过引用使用
fn takes_dyn(e: &dyn Error) {} // trait 对象同理
```

→ DST 详解：[04.1 DST 基础](./04-1-dst-basics.md) · [04.3 dyn/vtable](./04-3-dyn-vtable.md)

---

### 2. `Copy`

| 项 | 说明 |
|----|------|
| **契约** | 可按**二进制位**完整复制；复制后**原变量仍有效**（无 move） |
| **自动** | 所有字段均为 `Copy` 时 struct/元组自动实现 |
| **与 `Clone`** | 实现 `Copy` **必须**同时 `Clone`；`Clone` 显式、可深拷贝堆 |
| **反例** | `String`、`Vec` 有堆所有权 → **不能** `Copy`，只能 `Clone` |

```rust
let a = 10;
let b = a;        // Copy：a、b 都有效

let s1 = String::from("hi");
let s2 = s1;      // Move：s1 失效
// let s3 = s1.clone(); // Clone：两份堆数据
```

→ [Ch01 · 04.2 Move/Copy](../Chapter-01-Foundations/04-2-move-copy-clone.md)

---

### 3. `Send`

| 项 | 说明 |
|----|------|
| **契约** | 类型的**所有权**可安全**转移**到另一线程 |
| **自动** | 成员均 `Send` 的复合类型通常自动 `Send` |
| **反例** | 裸指针 `*mut T`、`Rc<T>`（非原子 refcount）→ **非** `Send` |

```rust
use std::rc::Rc;
use std::thread;

let r = Rc::new(1);
// thread::spawn(move || { r.clone(); }); // ❌ Rc 非 Send
```

---

### 4. `Sync`

| 项 | 说明 |
|----|------|
| **契约** | `&T` 可安全地在多线程间**共享**（并发读） |
| **等价** | `T: Sync` ⟺ `&T: Send` |
| **反例** | `RefCell<T>`（运行时内部可变、非线程安全）→ **非** `Sync` |
| **多线程写** | 用 `Mutex<T>` / `RwLock<T>` 等（`T: Send` 时常可 `Sync`） |

```rust
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

let cell = RefCell::new(0);
// Arc::new(cell); // RefCell 非 Sync → Arc<RefCell<_>> 非 Send/Sync 用于跨线程
```

→ 并发章：[RFR 第 10 章](../Chapter-10-Concurrency-and-Parallelism/README.md) · Nomicon [07 Send/Sync](../../03-Rust_Nomicon/07_Concurrency_Atomic/02-send-sync.md)

---

### 5. `Unpin`（异步 / `Pin`）

| 项 | 说明 |
|----|------|
| **契约** | 类型可安全**移动**内存（非「自引用结构」需钉住） |
| **默认** | 绝大多数类型自动 `Unpin` |
| **例外** | 部分 `async` 状态机内部可能 `!Unpin` |
| **用途** | `Pin<&mut T>` 体系；`!Unpin` 时不能随意 `mem::swap` 移动 |

→ 深入：[RFR 第 8 章 Async](../Chapter-08-Asynchronous-Programming/README.md)

---

## 三、对照总表

| Trait | 问的是什么 | 典型反例 / 注意 |
|-------|------------|-----------------|
| `Sized` | 编译期固定大小？ | `str`、`[T]`、`dyn Trait` |
| `Copy` | 赋值 = 位拷贝且原值仍有效？ | `String`、`Vec` |
| `Send` | 能 `move` 到别的线程？ | `Rc`、裸指针包装 |
| `Sync` | 能跨线程共享 `&T`？ | `RefCell`、`Cell` |
| `Unpin` | 能安全移动（非钉住自引用）？ | 部分 async 内部状态 |

---

## 四、自动实现 vs 手动 `unsafe impl`

### Auto Trait 自动推导

`Send`、`Sync`、`Sized`（对具体类型）、`Unpin` 等由编译器**扫描所有字段**推导：成员都满足 → 自动实现，无需手写 `impl`。

### 手动 `unsafe impl`

`Send`、`Sync` 是 **unsafe trait**（实现须 `unsafe impl`）：

```rust
struct MyRawPtrWrapper {
    ptr: *mut u8,
}

// 仅当你能证明跨线程安全时才写：
unsafe impl Send for MyRawPtrWrapper {}
unsafe impl Sync for MyRawPtrWrapper {}
```

| 项 | 说明 |
|----|------|
| **场景** | FFI 裸指针包装、自定义无锁结构、跨线程 OS 句柄 |
| **含义** | 开发者向编译器担保线程安全；违反 → **UB** |
| **孤儿规则** | 仍须类型或 trait 至少一方本地 → [07.1 孤儿规则](./07-1-orphan-rule.md) |

`Copy`、`Sized` 几乎不手动 `unsafe impl` — 用 `#[derive(Copy, Clone)]` 或语言内置规则。

---

## 五、核心设计目的

1. **编译期安全隔离** — `Send`/`Sync` 拒绝不安全跨线程类型，根源防数据竞争  
2. **内存布局约束** — `Sized` / `?Sized` 区分固定大小与 DST，是切片、`dyn Trait` 的基础  
3. **所有权语义** — `Copy` 区分隐式位拷贝与 move  
4. **零开销抽象** — 无运行时代码，检查全在编译期  

---

## 六、易混区分

| 易混 | 纠正 |
|------|------|
| Marker = 没 trait 方法 | `Iterator` 等有方法是**行为** trait；`Send` 是**契约** marker |
| `Sync` = 可以多线程写 | `Sync` 管 **`&T` 共享读**；写要用 `Mutex` 等 |
| 手动 `Send` 就能乱传指针 | 仍须保证指向内存在线程间合法 |
| Marker 不受孤儿规则约束 | **同样约束**；双外部不能 `impl Send for Foreign` |
| `Copy` 与 `Clone` 可互换 | `Copy` 隐式浅拷贝；`Clone` 显式、可堆复制 |

---

## 七、速记

1. **`Sized`：默认要；`?Sized` 给 DST。**  
2. **`Copy` 隐式位拷贝；`Send` 移线程；`Sync` 共享 `&T`。**  
3. **`Send`/`Sync` 手动 impl 必须 `unsafe` + 孤儿规则。**

---

## 对照阅读

- Book → [16.4 Send 与 Sync](../../00-Book/16-fearless-concurrency/16.4-Send与Sync.md)
- RFR → [第 10 章 并发](../Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md) · [第 8 章 Async / Unpin](../Chapter-08-Asynchronous-Programming/README.md)
- ER → [Item 10 标准 trait](../../01-ER/Chapter-02-Traits/Item-10-standard-traits/README.md)
- Nomicon → [07 Concurrency](../../03-Rust_Nomicon/07_Concurrency_Atomic/README.md)

→ 下一节：[10 存在类型](./10-existential-types.md)
