# 1.1 Naming Practices（命名惯例）

> 所属：**Unsurprising** · [← 章索引](./README.md)

← [章索引](./README.md) · 下一节 [02 通用 Trait](./02-common-traits-for-types.md)

Book → [7.2 引用项命名](../../00-Book/07-packages-modules/7.2-引用项命名.md)

---

优秀接口应符合社区直觉：用户「盲猜」用法时也应大概率猜对。

**原则**：复用 std / 生态约定俗成的名字；方法名应反映**是否消耗 `self`**、**返回借用还是拥有值**。

---

## 一、`as_` 系列 — 不拿所有权，只转引用

| 方法 | 作用 |
|------|------|
| **`as_ref()`** | `Option<T>` → `Option<&T>`；`Result<T,E>` → `Result<&T,&E>` — 原值仍可用 |
| **`as_mut()`** | 可变版 → `Option<&mut T>` 等 |
| **`as_ptr()` / `as_mut_ptr()`** | 转裸指针 — unsafe 场景 |

```rust
let x = Some(10);
let r = x.as_ref(); // Some(&10)，x 仍有效
```

> **口诀**：`as` 只借、不消耗本体。

---

## 二、`into_` 系列 — 拿走所有权，消耗原变量

调用后原变量**失效**（move），所有权转入返回值。

| 方法 | 典型 |
|------|------|
| **`into_inner()`** | `Mutex` / `Box` / 包装类型 → 取出内部值，销毁包装 |
| **`into_iter()`** | 消耗容器 → 产出 **owned** 元素的迭代器 |
| **`into_string()` / `into_vec()`** | 类型转换，消耗原值 |

> **口诀**：`into` = 交出自己，用完本体就没了。

→ 包装类型：[04 Wrapper Types](./04-wrapper-types.md)

---

## 三、`get_` 系列 — 安全借用，不 panic

返回 **`Option<&T>` / `Option<&mut T>`** — 越界 / 不存在 → `None`。

| 方法 | 说明 |
|------|------|
| **`get(index)`** | 切片 / 数组安全索引 |
| **`get_mut(index)`** | 可变引用版 |

对比：`arr[index]` 越界 **panic**；`get` 更安全。

---

## 四、`try_` 系列 — 可能失败，返回 `Result`

对标「必然成功」的版本；失败走 `Err`，**不 panic**。

| 方法 | 说明 |
|------|------|
| **`try_into()`** | 带错误的转换 — 对比 `Into::into()`（须保证成功） |
| **`try_lock()`** | 锁拿不到 → `Err`，不阻塞 |

---

## 五、`with_` 系列 — 构造 / 配置

新建对象时附带初始化参数；建造者模式常见。

| 方法 | 说明 |
|------|------|
| **`Vec::with_capacity(n)`** | 预分配容量 |
| **`Builder::with_xxx()`** | 链式配置 |

---

## 六、迭代器三巨头：`iter` / `iter_mut` / `into_iter`

| 方法 | 元素类型 | 容器所有权 | 用途 |
|------|----------|------------|------|
| **`iter()`** | `&T` | **不消耗** | 只读遍历 |
| **`iter_mut()`** | `&mut T` | **不消耗** | 遍历中修改 |
| **`into_iter()`** | `T` | **消耗**容器 | 拿走每个元素 |

### `iter()` — 不可变引用

```rust
let v = vec![1, 2, 3];
for x in v.iter() {
    println!("{}", x); // x: &i32
}
// v 仍可用
```

### `iter_mut()` — 可变引用

```rust
let mut v = vec![1, 2, 3];
for x in v.iter_mut() {
    *x *= 2;
}
```

### `into_iter()` — 所有权

```rust
let v = vec![1, 2, 3];
for x in v.into_iter() {
    println!("{}", x); // x: i32
}
// v 已 move，不可再用
```

### `for` 循环自动选择

```rust
for x in v {}           // 等价 v.into_iter() — 消耗 v
for x in &v {}          // 等价 v.iter()
for x in &mut v {}      // 等价 v.iter_mut()
```

→ 借用 vs 拥有：[07 Borrowed vs Owned](./07-borrowed-vs-owned.md)

---

## 七、整体速记对照表

| 前缀 / 方法 | 所有权 | 返回形式 |
|-------------|--------|----------|
| `as_ref` / `as_mut` | 借用，不消耗 | 包装后的引用 |
| `get` / `get_mut` | 借用，安全 | `Option` 引用 |
| `into_*` | 转移所有权 | 内部原值 / owned 迭代 |
| `try_*` | 通常不消耗 self | `Result` |
| `with_*` | 新建 | 配置完成的对象 |
| `iter` | 不可变借用 | `Item = &T` |
| `iter_mut` | 可变借用 | `Item = &mut T` |
| `into_iter` | 消耗容器 | `Item = T` |

---

## 八、反例（破坏直觉）

| 反例 | 问题 |
|------|------|
| `iter()` 却消耗所有权 | 与 std 语义冲突 |
| `get()` 与 `get_mut()` 不对称且无文档 | 增加挫败感 |
| 该 `try_` 却 panic | 调用方无法处理失败 |

---

## 九、可运行串讲示例

```rust
fn main() {
    // as_ref：不消耗
    let opt = Some(String::from("hi"));
    let r = opt.as_ref();
    assert_eq!(r, Some(&String::from("hi")));
    assert!(opt.is_some()); // opt 仍在

    // get：安全索引
    let arr = [10, 20, 30];
    assert_eq!(arr.get(1), Some(&20));
    assert_eq!(arr.get(99), None);

    // iter / iter_mut / into_iter
    let mut v = vec![1, 2, 3];
    let sum: i32 = v.iter().sum(); // 只读，v 仍可用
    assert_eq!(sum, 6);

    for x in v.iter_mut() {
        *x += 1;
    }
    assert_eq!(v, vec![2, 3, 4]);

    let v2 = vec![1, 2];
    let owned: Vec<i32> = v2.into_iter().collect(); // v2 已 move
    assert_eq!(owned, vec![1, 2]);
}
```

→ 速记：[01-cheat-sheet.md](./01-cheat-sheet.md) · 下一节：[02 通用 Trait](./02-common-traits-for-types.md)
