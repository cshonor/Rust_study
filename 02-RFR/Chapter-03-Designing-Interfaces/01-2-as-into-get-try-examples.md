# 1.1.2 · `as_` / `into_` / `get_` / `try_` 可运行详例

← [01 命名惯例](./01-naming-practices.md) · [01 速记](./01-cheat-sheet.md)

> 可运行 demo：`cargo run --manifest-path naming-series-demo/Cargo.toml`

---

## 四系列对照

| 系列 | 所有权 | 失败时 | 口诀 |
|------|--------|--------|------|
| **`as_`** | 借用，不消耗 `self` | — | 只借，本体还在 |
| **`into_`** | 消耗 `self` | — | 交出自己 |
| **`get_`** | 借用 | `None`，不 panic | 安全索引 |
| **`try_`** | 通常不消耗 | `Err`，不 panic | 可失败操作 |

---

## 一、`as_` 系列 — 只借不拿

典型：`String::as_str()`、`Option::as_ref()` — 返回引用视图，**原变量可反复调用**。

```rust
let s = String::from("hello");
let s1 = s.as_str();
let s2 = s.as_str(); // 原 s 还能用，不会被 move
println!("{}", s);   // hello
assert_eq!(s1, "hello");
assert_eq!(s2, "hello");
```

```rust
let x = Some(10);
let r1 = x.as_ref(); // Some(&10)
let r2 = x.as_ref();
assert!(x.is_some()); // x 仍在
assert_eq!(r1, r2);
```

```text
String ──as_str()──► &str     （s 仍拥有堆数据）
Option<T> ──as_ref()──► Option<&T>
```

> **注意**：`as_` 不转移所有权；需要「交出去」时用 `into_`。

---

## 二、`into_` 系列 — 消耗原变量

典型：`Vec::into_iter()` — 拿走容器所有权，产出 **owned** 元素。

```rust
let v = vec![1, 2, 3];
let mut v_iter = v.into_iter();
// println!("{:?}", v); // ❌ 编译错误：v 已被 into_iter move
assert_eq!(v_iter.next(), Some(1));
assert_eq!(v_iter.next(), Some(2));
assert_eq!(v_iter.next(), Some(3));
assert_eq!(v_iter.next(), None);
```

其它常见 `into_*`：

| 方法 | 效果 |
|------|------|
| `String::into_bytes()` | `String` → `Vec<u8>`，原串失效 |
| `Box::into_inner()` | 取出内部值，销毁 box |
| `Option::into_iter()` | 消耗 `Option`，迭代内部 `T` |

→ 包装类型：[04 Wrapper Types](./04-wrapper-types.md)

---

## 三、`get_` 系列 — `Option` 安全访问

越界 / 不存在 → **`None`**，不会像 `[index]` 那样 **panic**。

```rust
let v = vec![1, 2, 3];
assert_eq!(v.get(1), Some(&2));
assert_eq!(v.get(5), None); // 越界 → None，不崩溃
// let bad = v[5];          // ❌ panic: index out of bounds
```

| 对比 | 越界行为 | 返回类型 |
|------|----------|----------|
| `v[i]` | **panic** | `&T` |
| `v.get(i)` | `None` | `Option<&T>` |
| `v.get_mut(i)` | `None` | `Option<&mut T>` |

```rust
let mut v = vec![10, 20, 30];
if let Some(x) = v.get_mut(0) {
    *x += 1;
}
assert_eq!(v, vec![11, 20, 30]);
```

---

## 四、`try_` 系列 — `Result` 可失败操作

对标「必然成功」的 API；失败返回 **`Err`**，**不 panic、不阻塞**（视具体 API 而定）。

### `try_lock` — 拿不到锁就 `Err`

```rust
use std::sync::Mutex;

let lock = Mutex::new(5);
let mut guard = lock.lock().unwrap(); // 阻塞直到拿到锁
*guard = 10;
drop(guard); // 显式释放，下面演示同线程重入

// 锁仍被占用时 try_lock 立即返回 Err（不阻塞）
// 下面用新 scope 演示：guard 已 drop 后可再 lock
let guard2 = lock.try_lock().expect("锁应空闲");
assert_eq!(*guard2, 10);
```

**同作用域内锁未释放时**：

```rust
let lock = Mutex::new(5);
let _guard = lock.lock().unwrap();
assert!(lock.try_lock().is_err()); // 锁被 _guard 持有 → Err
```

### 其它 `try_*`

| 方法 | 对比 | 失败 |
|------|------|------|
| `try_into()` | `Into::into()` | 类型转换失败 → `Err` |
| `try_reserve()` | `reserve()` | 内存分配失败 → `Err` |

---

## 五、四系列串讲（可 `cargo run`）

```rust
fn main() {
    demo_as();
    demo_into();
    demo_get();
    demo_try();
}

fn demo_as() {
    let s = String::from("hello");
    let _ = s.as_str();
    let _ = s.as_str();
    assert_eq!(s, "hello");
}

fn demo_into() {
    let v = vec![1, 2, 3];
    let mut it = v.into_iter();
    assert_eq!(it.next(), Some(1));
}

fn demo_get() {
    let v = vec![1, 2, 3];
    assert_eq!(v.get(5), None);
}

fn demo_try() {
    use std::sync::Mutex;
    let lock = Mutex::new(5);
    let _g = lock.lock().unwrap();
    assert!(lock.try_lock().is_err());
}
```

→ demo 源码：[naming-series-demo/src/main.rs](./naming-series-demo/src/main.rs)

---

→ 速记：[01-2-cheat-sheet.md](./01-2-cheat-sheet.md) · `with_` 详例：[01-1 with/Builder](./01-1-with-series-and-builder.md)
