# 1.1.2 · `into_` 系列 — 所有权转移

> 所属：**Unsurprising / 命名惯例** · [← 01 命名 hub](./01-naming-practices.md)

← [01-1 `as_`](./01-1-as-series.md) · 下一节 [01-3 `get_`](./01-3-get-series.md)

Demo → `cargo run --manifest-path naming-series-demo/Cargo.toml into`

→ 包装类型：[04 Wrapper Types](./04-wrapper-types.md)

---

## 核心逻辑

| 点 | 说明 |
|----|------|
| **所有权** | **消耗** `self`，原变量**失效**（move） |
| **返回值** | 内部 owned 值、owned 迭代器、转换后类型 |
| **口诀** | `into` = 交出自己，用完本体就没了 |

```text
Vec<T> ──into_iter()──► impl Iterator<Item = T>   （v 不可再用）
String ──into_bytes()──► Vec<u8>
Box<T> ──into_inner()──► T
```

| 常见 API | 作用 |
|----------|------|
| **`into_iter()`** | 消耗容器 → **owned** 元素迭代器 |
| **`into_inner()`** | `Box` / `Mutex` 等 → 取出内部值 |
| **`into_string()` / `into_vec()`** | 类型转换，消耗原值 |

---

## 可运行示例

### `Vec::into_iter` — 容器被 move

```rust
let v = vec![1, 2, 3];
let mut v_iter = v.into_iter();
// println!("{:?}", v); // ❌ 编译错误：v 已被 into_iter move
assert_eq!(v_iter.next(), Some(1));
assert_eq!(v_iter.next(), Some(2));
assert_eq!(v_iter.next(), Some(3));
assert_eq!(v_iter.next(), None);
```

### `String::into_bytes`

```rust
let s = String::from("hi");
let bytes = s.into_bytes();
assert_eq!(bytes, b"hi");
// s 已失效
```

→ demo：`naming-series-demo/src/into_series.rs`

---

## 与迭代器三巨头

| 方法 | 元素 | 容器 |
|------|------|------|
| `iter()` | `&T` | 保留 |
| `iter_mut()` | `&mut T` | 保留 |
| **`into_iter()`** | **`T`** | **消耗** |

`for x in v` 等价 `v.into_iter()` — 见 [01 命名 §六](./01-naming-practices.md#六迭代器三巨头iter--iter_mut--into_iter)。

---

→ 速记：[01-2-cheat-sheet.md](./01-2-cheat-sheet.md) · 下一节：[01-3 `get_`](./01-3-get-series.md)
