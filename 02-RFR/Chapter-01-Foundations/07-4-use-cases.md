# 3.3.4 · 内部可变性应用场景

> 所属：**Borrowing and Lifetimes · 内部可变性** · [← 07 hub](./07-interior-mutability.md)

← [07.3 Cell/RefCell](./07-3-cell-vs-refcell.md) · 下一节 [07.5 对比与误区](./07-5-comparison-pitfalls.md)

---

## 场景 1 · `&self` 签名，内部改状态

trait / 回调常固定 **`&self`**，外部 `mut` 无法改字段 → 字段包 `RefCell` 或 `Cell`：

```rust
use std::cell::RefCell;

struct Counter {
    count: RefCell<u32>,
}

impl Counter {
    fn add(&self) {
        *self.count.borrow_mut() += 1;
    }
}
```

小计数可用 `Cell<u32>` + `set(get()+1)`。

---

## 场景 2 · `Rc<RefCell<T>>` 多所有者共享修改

```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(100));
let p1 = Rc::clone(&data);
let p2 = Rc::clone(&data);

*p1.borrow_mut() = 200;
println!("{}", p2.borrow());   // 200
```

`Rc` 只给 `&T`；`RefCell` 提供运行时内部可变 — **单线程**图/树经典组合。

---

## 场景 3 · 细粒度：整体不可变，局部字段可变

```rust
use std::cell::RefCell;

struct Cache {
    key: String,           // 核心：无 mut 不可换绑
    hits: RefCell<u32>,    // 仅统计可改
}
```

外部 `let cache` 不必 `mut`；误改 `key` 仍被编译器拦住。

---

## 场景 4 · 多句柄共享同一 `RefCell`

```rust
let data = RefCell::new(5);
let r1 = &data;
let r2 = &data;

{
    let _a = r1.borrow_mut();
    // let _b = r2.borrow_mut(); // ❌ panic
}
let _c = r2.borrow(); // ✅ mut 借用已结束
```

编译期：多个 `&RefCell` 合法；运行时：`borrow`/`borrow_mut` 仍互斥。

---

## 场景 5 · `RefCell` + 非 `Copy` 原地修改

```rust
let cell = RefCell::new(vec![1, 2, 3]);
let mut w = cell.borrow_mut();
w.push(4);
```

`Cell` 做不到 `push` — 只能整体 `set` 新 `Vec`。

---

## 选型速查

| 需求 | 选型 |
|------|------|
| `&self` + 小整数计数 | `Cell<u32>` |
| `&self` + `String`/`Vec` | `RefCell<T>` |
| 多 `Rc` 共享写 | `Rc<RefCell<T>>` |
| 多线程 | `Arc<Mutex<T>>` 等 |

→ 下一节：[07.5 对比与误区](./07-5-comparison-pitfalls.md)
