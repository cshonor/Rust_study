# 3.3.1 · Cell\<T\> 与 RefCell\<T\> 完整详解

> 所属：**Borrowing and Lifetimes · 内部可变性** · [← 07 hub](./07-interior-mutability.md)

← [07 内部可变性总览](./07-interior-mutability.md) · 下一节 → [08 生命周期](./08-lifetimes.md)

---

## 前置

二者都是 **`std::cell` 单线程内部可变性容器**：

- 外层常用 `let`（绑定不可变），仍可改**盒内**数据；
- 借用检查从**编译期**部分转移到**运行时**（`RefCell`）或靠 **Copy 规避借用**（`Cell`）；
- 共同底层：**`UnsafeCell<T>`** — 语言唯一允许 opt-out 静态「`&T` 永不修改」假设的类型。

→ 总览 [07 §零～§二](./07-interior-mutability.md)

---

## 一、`Cell<T>`

### 1. 适用范围

仅 **`T: Copy`**：

| ✅ 常见 | ❌ 不行 |
|---------|---------|
| `i32` `u64` `bool` `char` | `String` `Vec` |
| `(i32, i32)` 等小型 Copy 元组 | 自定义非 Copy 结构体 |

### 2. 核心 API

| 方法 | 作用 |
|------|------|
| `Cell::new(val)` | 创建并装入初始值 |
| `.get()` | **拷贝**出一份 `T`（不产生引用） |
| `.set(new_val)` | 用新值**整体覆盖**内部 |
| `.replace(new_val)` | 放入新值，**返回**旧值副本 |

### 3. 工作原理

```text
读：get()  → 复制 T 给你，无活跃 &T / &mut T
写：set()  → 直接覆盖盒内，无借用冲突
```

**全程不产生任何引用** — 因此**没有** borrow 计数，**不会**因借用冲突 panic。

### 4. 优缺点

| 优点 | 缺点 |
|------|------|
| **零 panic 风险**（无借用冲突） | 只能 `Copy` |
| 无计数器，开销极低 | 读写皆拷贝，大 `Copy` 结构仍可能贵 |
| API 最简单 | 不能 `borrow` 内部做分段修改 |

### 5. 示例

```rust
use std::cell::Cell;

fn main() {
    let num = Cell::new(10);   // 外层 let，绑定不可变

    num.set(20);
    println!("{}", num.get()); // 20

    let old = num.replace(30);
    println!("旧值：{}，新值：{}", old, num.get());
}
```

### 6. 典型场景

- 简单**计数器**、标志位（`Cell<bool>`）
- `Rc` 内嵌小 Copy 字段（不如 `RefCell` 常见，但可行）
- 图算法里「访问标记」等只需整体替换的 `Copy` 状态

---

## 二、`RefCell<T>`

### 1. 适用范围

**任意 `T`** — `String`、`Vec`、自定义结构体、堆上复杂数据。

### 2. 核心 API

| 方法 | 返回 | 说明 |
|------|------|------|
| `RefCell::new(val)` | `RefCell<T>` | 创建 |
| `.borrow()` | `Ref<T>` | 共享只读；**多个可共存** |
| `.borrow_mut()` | `RefMut<T>` | 独占可变；**读计数=0 且无写** |
| `.try_borrow()` | `Result<Ref<T>, _>` | 失败返回 `Err`，**不 panic** |
| `.try_borrow_mut()` | `Result<RefMut<T>, _>` | 同上 |

`Ref` / `RefMut` 是**智能守卫**：析构时自动减少计数（类似 `drop(r1)`）。

### 3. 工作原理

```text
RefCell {
    borrow_state:  RUNNING 时统计  读+ / 写独占标记
    value:         UnsafeCell<T>
}

borrow()      → 若无活跃写，读计数 +1
drop(Ref)     → 读计数 -1
borrow_mut()  → 读计数必须为 0 且无写；否则 panic
RefMut 存活   → 禁止任何 borrow / borrow_mut
```

### 4. 优缺点

| 优点 | 缺点 |
|------|------|
| **任意类型**，无 `Copy` 限制 | 维护借用状态，轻微运行时开销 |
| `borrow_mut` 可**原地**改 `String`/`Vec` | 冲突 → **panic**（或用 `try_*`） |
| 单线程最通用的内部可变容器 | **非** `Sync`，不能跨线程（用 `Mutex`） |

### 5. 示例

```rust
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello"));

    let r1 = s.borrow();
    let r2 = s.borrow();
    println!("{} {}", r1, r2);   // ✅ 多读共存

    drop(r1);
    drop(r2);

    let mut w = s.borrow_mut();
    w.push_str(" world");
    println!("{}", w);
}
```

### 6. `try_borrow` 避免 panic

```rust
let cell = RefCell::new(vec![1, 2, 3]);
let _guard = cell.borrow();

match cell.try_borrow_mut() {
    Ok(mut v) => v.push(4),
    Err(_) => eprintln!("已有活跃借用，跳过"),
}
```

---

## 三、`Cell` vs `RefCell` 对照表

| 维度 | `Cell<T>` | `RefCell<T>` |
|------|-----------|--------------|
| **`T` 限制** | 仅 `Copy` | 任意 `T` |
| 读写方式 | `get`/`set` **整体拷贝** | `borrow`/`borrow_mut` **引用** |
| 运行时检查 | **无**（永不因借用 panic） | 借用计数；冲突 **panic** |
| 性能 | 极小 | 计数增减（通常可忽略） |
| 分段改内部 | ❌ 只能整体替换 | ✅ `RefMut` 自由改字段/ `push` |
| 典型场景 | 小整数、bool 计数 | `String`/`Vec`/结构体、`Rc<RefCell<_>>` |

---

## 四、底层共性

1. **`std::cell`** — **单线程**；多线程用 `Mutex`/`RwLock`（`Sync`）。  
2. **内部可变性** — 外层 `let` 不换绑，改盒内。  
3. **`UnsafeCell<T>`** — 安全封装在容器内，用户不直接碰。  
4. **不替代 `let mut`** — 无共享句柄、路径清晰时优先外部可变（编译期、零开销）。

---

## 五、选型指南

```text
小 Copy 值，整体 get/set 够用     →  Cell
String / Vec / 结构体，要借用修改  →  RefCell
Rc 多所有者共享写（单线程）         →  Rc<RefCell<T>>
多线程共享写                        →  Mutex<T> / RwLock<T>
能 let mut + &mut，无 &self 约束    →  不用 cell，优先外部可变
```

| 组合 | 用途 |
|------|------|
| `Cell<u32>` in `&self` 方法 | 轻量计数 |
| `RefCell<Vec<_>>` | 共享句柄下推元素 |
| `Rc<RefCell<T>>` | 图、树、多指针改同一节点 |

→ 场景展开 [07 §五](./07-interior-mutability.md#五三大核心场景)

---

## 六、关键误区

| 误区 | 纠正 |
|------|------|
| Cell/RefCell = 随便乱改 | `Cell` 靠拷贝无冲突；`RefCell` 运行时仍互斥，违规 **panic** 非 UB |
| 可替代所有 `let mut` | 能静态检查时用 **`let mut`** 更安全、零开销 |
| `Cell` 能存 `String` | **编译失败** — `String` 非 `Copy` |
| `RefCell` 多线程安全 | 仅单线程；跨线程 → `Mutex` |
| `get()` 拿到内部引用 | `Cell` 只有**值副本**，无 `&T` 指向盒内 |

---

## 七、与外部可变性一句话

| | **外部 `let mut` + `&mut`** | **`Cell`** | **`RefCell`** |
|---|---------------------------|------------|---------------|
| 检查 | 编译期 | 无借用检查 | 运行时借用 |
| 冲突 | 编译错误 | 无（拷贝） | panic |
| 何时优先 | 默认首选 | 小 Copy + `&self` | 复杂 `T` + `&self` / `Rc` |

---

## 八、速记

1. **`Cell`：Copy only，`get`/`set`，永不 panic。**  
2. **`RefCell`：任意 T，`borrow`/`borrow_mut`，冲突 panic。**  
3. **都能外层 `let` 改盒内；底层都是 `UnsafeCell`。**

---

## 自测

- [ ] 为什么 `Cell` 不需要借用计数？  
- [ ] `RefCell<String>` 如何用 `push_str` 而不整体 `set`？  
- [ ] 何时选 `try_borrow_mut` 而不是 `borrow_mut`？  
- [ ] `Rc<RefCell<T>>` 解决了哪两个独立问题？

→ 返回 [07 内部可变性总览](./07-interior-mutability.md) · 下一节 [08 生命周期](./08-lifetimes.md)
