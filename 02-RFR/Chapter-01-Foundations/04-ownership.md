# 2. Ownership（所有权）

> 所属：**Ownership** · [← 章索引](./README.md)

所有权保证：每个值在某一责任域内被**唯一**地负责释放（**RAII** / **`Drop`**）。

物理背景 → [03.1 Rust 内存模型 · 栈/堆/静态](./03-1-rust-memory-model.md) · 术语 → [01 内存术语](./01-memory-terminology.md)

---

## 三条核心规则

| # | 规则 | 含义 |
|---|------|------|
| 1 | **每个值有且只有一个所有者** | 同一时刻只有一个绑定「负责 drop」；避免多指针重复释放、悬垂 |
| 2 | **所有者离开作用域时，值被 drop** | RAII：自动调 `Drop`，释放堆/关闭句柄等 |
| 3 | **所有权可转移（move）；非 `Copy` 类型不能隐式复制** | `let s2 = s1;` 后 `s1` 失效（若 `String`）；`i32` 等 `Copy` 类型除外 |

```rust
let s1 = String::from("hello");
let s2 = s1;   // move：堆上字符串的所有权 s1 → s2
// println!("{s1}"); // ❌ s1 不再负责这块堆
```

---

## Move 与 Copy

| | **Move** | **Copy** |
|---|----------|----------|
| **典型类型** | `String`、`Vec`、`Box<T>` | `i32`、`bool`、`char`、`*const T`（部分） |
| **赋值时** | 所有权转移；旧绑定失效 | 按位复制栈上表示；**两个绑定都有效** |
| **实现** | 默认（未实现 `Copy`） | `impl Copy for T` + 满足 Copy 约束 |
| **与 Drop** | 不能对需自定义 `Drop` 的类型自动 `Copy` | Copy 类型通常无自定义 Drop |

```rust
let a = 5;
let b = a;      // Copy：a、b 都有效
println!("{a} {b}");

let v = vec![1, 2];
let w = v;      // move
// println!("{v:?}"); // ❌
```

**易错**：「赋值 = 复制」在 Rust 里**仅对 `Copy` 成立**；owned 类型默认是 **move**。

---

## Drop 是什么

- 类型可实现 **`Drop` trait**：离开作用域时编译器插入 **`drop()`** 调用。
- **`String` / `Vec` / `Box`**：drop 时释放堆（见 [03.1](./03-1-rust-memory-model.md)）。
- **引用 `&T` / `&mut T`**：**不拥有**值，drop 引用**不会**释放被指向的数据；只有**所有者** drop 时才释放。

```rust
let x = String::from("hi");
let k = &x;     // k 是借用，不是所有者
// 作用域结束：只 drop x（释放堆）；k 作为引用无堆责任
```

---

## Drop 顺序（重点 · 易混）

**核心结论**：局部变量的销毁遵循栈 **LIFO（后进先出）** — **后声明 → 先 Drop；先声明 → 后 Drop**。这和声明顺序**正好相反**。

复合类型（struct / 元组）**内部字段**则按**定义正序** Drop — 与局部变量**方向相反**，最易混。

---

### 1. 局部变量、函数参数：栈 LIFO → **逆声明顺序**

后压栈的先出栈：**后声明的先 drop**。

```rust
fn main() {
    let a = String::from("A"); // 先声明，先入栈（栈底侧）
    let b = String::from("B"); // 后声明，后入栈（栈顶侧）
} // 出栈：Drop(b) → Drop(a)
```

**原因**：后声明的绑定可能依赖先声明的（`let a = ...; let r = &a;`）。若先 drop `a`，`r` 可能悬垂 → 故**先结束 / drop 后声明的**，再 drop 先声明的所有者。

```rust
let x = String::from("x");
let r = &x;   // r 后声明 → r 先结束 → 再 drop x
```

**引用 `&T` / `&mut T`**：无所有权，**不触发**对被指向值的 Drop；只影响「借用何时结束」，不改变所有者 drop 的时机。

---

### 2. 嵌套作用域：内层先于外层（仍是 LIFO）

内层块里的变量在外层之前销毁：

```rust
fn main() {
    let outer = String::from("外");

    {
        let inner = String::from("内");
    } // 块结束：Drop(inner) — 立刻

} // 函数结束：Drop(outer)
```

执行顺序：`Drop(inner)` → … → `Drop(outer)`。

---

### 3. 复合类型内部字段：**正序（定义顺序）**

结构体、元组、数组元素等：**先定义的字段先 drop**（与 §1 局部变量**相反**）。

```rust
struct Foo {
    x: String, // 先定义
    y: String, // 后定义
}

fn main() {
    let f = Foo { x: "X".into(), y: "Y".into() };
} // Drop(x) → Drop(y)
```

---

### 对照（面试常考）

| 场景 | Drop 顺序 | 记忆 |
|------|-----------|------|
| **函数内多个局部变量** | 逆声明（栈 LIFO） | 后声明先死 |
| **嵌套 `{ }` 块** | 内层先于外层 | 小块先结束 |
| **同一 struct / 元组字段** | 正定义序 | 先定义先死 |
| **函数参数** | 逆参数列表顺序 | 同局部变量 |

**⚠️ 局部变量（逆）vs struct 字段（正）— 方向相反。**

### 一句话区分

1. **函数里单独的 `let`**：栈 LIFO → **逆声明顺序**销毁  
2. **struct / 元组成员**：按源码定义顺序 → **正序**销毁  
3. **`&T`**：不拥有、不 Drop 目标，只结束借用

---

## 自定义 `Drop`：亲眼看到逆序

```rust
struct Custom {
    name: String,
}

impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping Custom: {}", self.name);
    }
}

fn main() {
    let a = Custom { name: "A".into() };
    let b = Custom { name: "B".into() };
}
// 输出：
// Dropping Custom: B
// Dropping Custom: A
```

---

## `Box<T>`、嵌套类型的 drop

```rust
let b = Box::new(String::from("hi"));
// drop 时：先 drop Box 管理的堆上 String，再清理 Box 自身（栈上句柄）
```

- **`Box<String>`**：栈上 `Box` 句柄 + 堆上 `String`；drop **`Box` 时**按 `Box` 的规则释放堆上 `T`。
- **struct 多字段**：外层 struct 按字段正序 drop；每个字段再按自己的类型规则 drop（如 `String` 释堆）。

---

## 引用与 move

| | 是否拥有 | 赋值时 | 作用域结束 |
|---|----------|--------|------------|
| **`String` / `Box<T>`** | 拥有 | 默认 **move** | **Drop** 释放资源 |
| **`&T` / `&mut T`** | 不拥有 | **Copy**（复制指针+生命周期） | 只结束借用，**不** drop 目标 |

→ 借用细节 [05 共享引用](./05-shared-references.md) · [06 可变引用](./06-mutable-references.md)

---

## panic、unwind 与 Drop

| 情况 | Drop 行为 |
|------|-----------|
| **正常 return** | 按上述顺序 drop 栈上变量 |
| **panic + unwind**（默认） | 栈展开，**仍按正常 Drop 顺序**析构已构造的对象 → 一般不泄漏 |
| **`catch_unwind`** | 捕获 panic；**被展开帧里的 Drop 已执行** |
| **abort 模式**（`panic=abort`） | **不 unwind**，栈上 Drop **可能不跑** → 依赖 OS 回收 |

```rust
use std::panic;

let r = panic::catch_unwind(|| {
    let s = String::from("leak?");
    panic!("boom");
});
// unwind 路径上 s 仍会被 drop
```

**实践**：需要 panic 时也释放锁/文件 → 用 `Drop` / RAII；关键资源勿假设 abort 下会 drop。

---

## 易错点速查

| 易错 | 正确理解 |
|------|----------|
| 赋值后原变量还能用 | 仅 **`Copy`** 类型；`String` 等是 **move** |
| 局部变量与 struct 字段 drop 同序 | **相反**：局部逆声明，字段正定义 |
| `&x` 会 drop `x` | **不会**；只有所有者 drop |
| panic 一定泄漏 | **unwind 默认会 drop**；`abort` 除外 |
| `Copy` 与自定义 `Drop` | 有自定义 **`Drop` 则不能 `Copy`** |

---

## 延伸（深入另读）

| 主题 | 去向 |
|------|------|
| `ManuallyDrop`、故意泄漏 | Nomicon · [第 9 章 Unsafe](../Chapter-09-Unsafe-Code/README.md) |
| `Pin` 与位置不可移动 | [第 8 章 Async](../Chapter-08-Async/README.md) |
| `async` 状态机里的 drop / cancel | [08 Async](../Chapter-08-Async/README.md) |
| RAII 惯用法 | ER [Item 11 Drop/RAII](../../01-ER/Chapter-02-Traits/Item-11-drop-raii/README.md) |

---

## 对照阅读

- Book → [4.1 什么是所有权](../../00-Book/04-ownership/4.1-什么是所有权.md)
- 内存三分类 → [03.1 Rust 模型](./03-1-rust-memory-model.md)
- 下一节 → [05 共享引用](./05-shared-references.md)（借用不抢所有权）
