# Item 2 · 案例与代码

← [Item 2 目录](../README.md)

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
