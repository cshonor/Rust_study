# Item 9: Consider using iterator transforms instead of explicit loops

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：考虑用迭代器转换替代显式循环  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 闭包（迭代器里常用） | [13.1 闭包](../../Book/13-iterators-closures/13.1-闭包.md) |
| `Iterator`、`map`/`filter`/`collect` | [13.2 迭代器](../../Book/13-iterators-closures/13.2-使用迭代器处理元素序列.md) |
| 循环 vs 迭代器性能 | [13.4 性能对比](../../Book/13-iterators-closures/13.4-性能对比-循环-vs-迭代器.md) |
| `Option`/`Result` 链式 | [Item 3](../Item-03-option-result-transforms/README.md) |
| 控制流 `for` | [3.5 控制流](../../Book/03-common-concepts/3.5-控制流.md) |

---

## 1. 核心知识点与关键定义

### `Iterator` trait

- 核心方法：**`next() -> Option<Item>`**（`None` 表示耗尽）。
- **`for item in iter`**：语法糖，反复 `next` 并绑定元素。

### 适配器（Adaptors，惰性）

| 类别 | 示例 |
|------|------|
| 流程 | `take`、`skip`、`chain`、`cycle` |
| 映射 | `map`、`cloned`、`enumerate`、`zip` |
| 过滤 | `filter`、`filter_map`、`flatten` |

### 消费者（Consumers，终止迭代）

`for_each`、`sum`、`fold`、`find`、`any`、`all`、**`collect`** 等。

### 三种迭代方式

| 写法 | 产出 | 集合 |
|------|------|------|
| `into_iter()` / `for x in v` | 按值 / 拥有项 | **消费** `v` |
| `iter()` / `for x in &v` | `&Item` | 保留 `v` |
| `iter_mut()` | `&mut Item` | 保留 `v`，可改元素 |

---

## 2. 逻辑脉络

```text
while → 下标 for → for-each → 迭代器链（源 + 适配器 + 消费者）
```

### 三段论

1. **源**：`values.iter()` 等  
2. **适配器链**：`filter` → `take` → `map`（**惰性**，直到消费者才跑）  
3. **消费者**：`sum()` / `collect()`  

### 性能与安全

- 下标 `values[i]` → 运行时**边界检查**。
- 迭代器遍历在 LLVM 优化下常能**消除**边界检查 → 有时比手写下标循环更快（见 Book 13.4，仍应 **bench** 验证）。

---

## 3. 重点结论与实用要点

1. **优先迭代器链**——更惯用、意图更清晰（过滤/映射/聚合各就各位）。
2. **`collect::<Result<Vec<_>, _>>()`**——把 `Iterator<Item = Result<T,E>>` 收成 **`Result<Vec<T>, E>`**；首个 `Err` 即停；配合 `?` 向上传。
3. **何时保留显式 `for`**——循环体巨大、多职责、需复杂 early return 时，硬塞进闭包反而难读。
4. **别盲目迷信**——热点路径用 `cargo bench` / Godbolt 对比显式循环与迭代器。

---

## 4. 案例与代码要点

### 前 5 个偶数的平方和

```rust
// 演进：iter → filter → take → map → sum
let even_sum_squares: u64 = values
    .iter()
    .filter(|x| *x % 2 == 0)
    .take(5)
    .map(|x| x * x)
    .sum();
```

### `Option` / `Result` 当迭代器 + `flatten`

```rust
let opts = vec![Some(1), None, Some(3)];
let nums: Vec<_> = opts.into_iter().flatten().collect(); // [1, 3]

// Result 同理：flatten 掉 Err（或配合 filter_map）
```

### `Result` 集合一次 collect

```rust
let parsed: Result<Vec<i32>, _> = strings
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();
let vec = parsed?; // 任一 parse 失败则 ? 返回
```

---

## 5. 易错细节

| 问题 | 说明 |
|------|------|
| **`for x in collection`** | 默认 **move** / `into_iter`，后面不能用 `collection` |
| 需保留集合 | `for x in &collection` 或 `.iter()` |
| **`collect()` 类型** | 编译器常推不出容器类型 → `collect::<Vec<_>>()` 或 `let v: Vec<_> = ...` |
| 闭包捕获 | `filter(\|x\| ...)` 中 `x` 是 `&&T`，比较时常要 `*x` |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 09](../ER-拓展索引.md#item-09)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 结构 | 源 + 适配器（懒） + 消费者 |
| 借用 | `iter()` → `&T`；`into_iter()` 吃掉集合 |
| Result 流 | `collect::<Result<Vec<_>, _>>()` |
| 可读性 | 大循环体别硬链式 |
| 性能 | 零成本抽象，但仍要测 |
