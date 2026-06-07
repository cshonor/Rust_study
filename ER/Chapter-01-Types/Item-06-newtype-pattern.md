# Item 6: Embrace the newtype pattern

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：拥抱 newtype 模式  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-05-06-newtype](../ER-demos/item-05-06-newtype/)（derive_more）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 元组结构体 | [5.1 结构体](../../Book/05-structs/5.1-定义并实例化结构体.md) |
| 孤儿规则、`Error` | [Item 4](./Item-04-idiomatic-error-types.md) |
| `From` / `Into` 单位转换 | [Item 5](./Item-05-type-conversions.md) |
| `Deref` 减少 `.0` | [15.2 Deref](../../Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) · [15.2.1 勿滥用](../../Book/15-smart-pointers/15.2.1-Deref嵌套可变与编译坑.md#反例-3过度-derefeffective-rust-item-6) |
| 布尔参数 → 枚举 | [Item 1](./Item-01-express-data-structures.md) |

---

## 1. 核心知识点与关键定义

### Newtype 模式

- 用**单字段元组结构体**包裹已有类型 → 在类型系统里得到**新的独立类型**，值域相同、**类型不同**。

```rust
pub struct UserId(u64);
pub struct OrderId(u64); // 与 UserId 不能混用
```

### 孤儿规则（Orphan Rule）

- 只有 **trait 或类型至少一方定义在当前 crate**，才能 `impl Trait for Type`。
- 不能为外部库的 `StdRng` 直接 `impl Display`（类型与 trait 都在外部）。

### 类型别名 vs Newtype

| | `type Alias = T` | `struct New(T)` |
|--|------------------|-----------------|
| 编译器视角 | **仍是 `T`** | **新类型** |
| 混用检查 | ❌ 不防呆 | ✅ `mismatched types` |
| 自定义 trait | 不能为新类型 impl | ✅ 本地类型可 impl |

---

## 2. 逻辑脉络与知识点关联

```text
裸 f64 / bool（语义易混）
  → type 别名（仅文档，无安全）
  → Newtype（编译期防混）
  → 必要时 From/TryFrom 做合法转换
  → 外部类型 + 外部 trait → Newtype 突破孤儿规则
```

---

## 3. 重点结论与实用要点

1. **强业务语义、易混单位/ID** → 用 Newtype，别只靠 `type` 别名。
2. **FFI / 布局等价** → `#[repr(transparent)]`，与内部类型内存布局一致，零开销。
3. **为第三方类型 impl 标准库/serde 等 trait** → 包一层本地 Newtype 是标准做法。
4. **配合 Item 1**：多 `bool` 参数歧义也可优先 **enum**，Newtype 适合「同底层、不同语义」。

---

## 4. 案例与代码要点

### 物理单位防呆（火星探测器）

```rust
// ❌ 别名：编译器不拦混用
type PoundForceSeconds = f64;
type NewtonSeconds = f64;

// ✅ Newtype：混用即报错
pub struct PoundForceSeconds(pub f64);
pub struct NewtonSeconds(pub f64);

fn thrust(_: NewtonSeconds) {}

// thrust(PoundForceSeconds(1.0)); // mismatched types

impl From<PoundForceSeconds> for NewtonSeconds {
    fn from(p: PoundForceSeconds) -> Self {
        NewtonSeconds(p.0 * 4.448) // 示意换算
    }
}
```

### 孤儿规则：`Display for StdRng`

```rust
// impl fmt::Display for rand::rngs::StdRng { ... } // ❌ E0117

struct MyRng(rand::rngs::StdRng);

impl fmt::Display for MyRng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<MyRng instance>")
    }
}
```

### `#[repr(transparent)]`（示意）

```rust
#[repr(transparent)]
pub struct Wrapper(pub Inner);
// 与 Inner 同布局，适合 FFI / 透明传递
```

---

## 5. 易错细节

### 访问内部值

- 字段通过 **`.0`**（或具名字段）访问，比裸类型多一层。

### Trait 不会自动继承

Newtype **不**自动拥有内部类型的 trait 实现：

| 恢复方式 | 适用 |
|----------|------|
| `#[derive(Clone, Debug, Eq, …)]` | 可 derive 的 trait |
| 手写转发 `self.0.fmt(f)` | `Display` 等复杂 trait |
| `Deref` / `derive_more` | 减少 `.0` 与转发样板（见 §6） |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 06](../ER-拓展索引.md#item-06)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| Newtype | 单字段 tuple struct = 新类型 |
| vs `type` | 别名不防混，Newtype 防混 |
| 孤儿 | 包成 local type 再 impl trait |
| `repr(transparent)` | 与内部同布局 |
| trait | 不继承，derive 或手写转发 |
