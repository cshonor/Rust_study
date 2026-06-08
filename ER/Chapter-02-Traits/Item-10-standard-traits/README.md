# Item 10: Familiarize yourself with standard traits

> **Effective Rust** · [Chapter 2 — Traits](../ER-本书目录.md)  
> **中文**：熟悉标准 trait  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| trait 基础 | [10.2 trait](../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| `Copy` / 移动 | [4.1 所有权](../../Book/04-ownership/4.1-什么是所有权.md) |
| `Default` | [3.1 变量和可变性](../../Book/03-common-concepts/3.1-变量和可变性.md) |
| `HashMap` 需要 `Hash + Eq` | [8.3 HashMap](../../Book/08-collections/8.3-hashmap.md) |
| 操作符 trait | [19.2 高级 trait](../../Book/19-advanced-features/19.2-高级trait.md) |
| 常用 trait 一览 | [Item 2](../Item-02-express-common-behavior/README.md) §标准 trait |

---

## 1. 核心知识点与关键定义

### `#[derive(...)]`

- 为 struct / enum **自动生成**标准 trait 的默认实现（通常逐字段 / 逐变体组合规则）。

### 复制与拷贝

| Trait | 含义 |
|-------|------|
| **`Clone`** | 可 `.clone()` 显式复制（可含自定义逻辑） |
| **`Copy`** | **标记 trait**：按位拷贝即安全；赋值走 **Copy 语义**而非 **Move** |

### 等价与比较

| Trait | 能力 |
|-------|------|
| **`PartialEq`** | `==` / `!=`（部分等价） |
| **`Eq`** | 标记：`PartialEq` + **反射性** `x == x` |
| **`PartialOrd`** | 部分序：`<`、`>` 等（可能不可比） |
| **`Ord`** | 全序：`PartialOrd` + `Eq` + 任意两值可比较 |

### 展示

| Trait | 格式化 | derive |
|-------|--------|--------|
| **`Debug`** | `{:?}`，给程序员 | ✅ 可 derive |
| **`Display`** | `{}`，给用户 | ❌ 须手写 |

### 其它常用

| Trait | 作用 |
|-------|------|
| **`Default`** | 合理默认值 / 空值 |
| **`Hash`** | 稳定哈希 → `HashMap` / `HashSet` 的 key |

---

## 2. 逻辑脉络与层级

```text
Clone ← Copy（Copy 必须 Clone）
PartialEq ← Eq（Eq 是标记）
PartialEq + PartialOrd ← Ord
Eq + Hash → 必须 hash(x)==hash(y) when x==y
```

### 浮点数的鸿沟

- IEEE 754：**`NaN != NaNaN`** → 无反射性。
- 含 **`f32` / `f64`** 的类型 **不能** derive **`Eq` / `Ord`**（只能 `PartialEq` / `PartialOrd`）。

---

## 3. 重点结论与实用要点

1. **`Copy` 要克制**——小、纯数据、按位拷贝便宜；大 struct 的隐式 Copy 可能拖性能；含 `&mut T` 等不能 Copy。
2. **尽量 derive `Debug`**——除非敏感字段；可 `#![warn(missing_debug_implementations)]`。
3. **`Default` + `..Default::default()`**——多可选字段初始化（与 Item 7 Builder 互补）。
4. **`std::ops` 重载要一致**——有 `Add` + `Neg` 则 `Sub` 应满足 `x - y ≈ x + (-y)`；别给无关类型乱重载。
5. **手写 `PartialEq` 时别忘 `Eq`**——若逻辑满足反射性，补 `impl Eq for T {}` 才能当 `HashMap` key。

---

## 4. 案例与代码要点

### `Copy` vs Move

```rust
#[derive(Clone, Copy, Debug)]
struct KeyId(u64);

let k = KeyId(42);
let k2 = k;              // 按位拷贝
println!("{:?}", k);     // ✅ 仍可用

// 无 Copy 时：let k2 = k; 后 k 不能再 use
```

### 浮点 `PartialOrd` 陷阱

```rust
#[derive(PartialOrd, PartialEq)]
struct Oddity(f32);

let x = Oddity(f32::NAN);
// x <= x 可能为 false —— 别假设自比较恒 true
```

### `Hash` + `Eq` 契约

```rust
// 若 x == y，必须 hash(x) == hash(y)
// 手写其一通常要手写或 derive 另一，且逻辑一致
```

---

## 5. 易错细节

| 问题 | 说明 |
|------|------|
| **`Copy` 上 `.clone()`** | 多余；Clippy `clone_on_copy` |
| **derive `Ord` 顺序** | 按**字段声明顺序**字典比较，未必符合业务权重 → 手写 |
| **只 impl `PartialEq`** | 缺 `Eq` → 不能作需要 `Eq` 的 key |
| **含浮点求全序** | 不能 `Ord`；排序前过滤 NaN 或用手动规则 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 10](../ER-拓展索引.md#item-10)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| Trait | 一句 |
|-------|------|
| `Copy` | 小纯数据；改 Move→按位拷 |
| `Debug` / `Display` | `:?` 可 derive；`{}` 手写 |
| `Eq` | 标记 + 反射；浮点不行 |
| `Hash` + `Eq` | 相等则 hash 相等 |
| `Ord` derive | 字段顺序 ≠ 业务优先级 |
