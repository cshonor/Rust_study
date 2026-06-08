# Item 5: Understand type conversions

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：理解类型转换  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-05-06-newtype](../Item-05-type-conversions/demo/)（Deref 强制转换）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 数值、`as`、类型 | [3.2 数据类型](../../Book/03-common-concepts/3.2-数据类型.md) |
| `Deref` 强制转换 | [15.2 Deref](../../Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) · [15.2.1](../../Book/15-smart-pointers/15.2.1-Deref嵌套可变与编译坑.md) |
| `From` / `Into` 与 `?` | [Item 4](../Item-04-idiomatic-error-types/README.md)、[9.2 Result](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| trait 对象强制转换 | [17.2 trait 对象](../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| Newtype 转换 | [Item 6](../Item-06-newtype-pattern/README.md)（ER） |

---

## 1. 核心知识点与关键定义

### 三类转换机制

| 类别 | 机制 | 示例 |
|------|------|------|
| **Manual（手动）** | 实现 `From` / `TryFrom` 等 trait | `MyType::from(x)` |
| **Semi-automatic（半自动）** | `as` 显式强转 | `x as u16` |
| **Automatic（自动 / Coercion）** | 编译器静默强制转换 | 见 §5 |

### 无隐式数值转换

- Rust **不在数值类型间做自动隐式转换**。
- 即使 `u32` → `u64` 看似「安全」，也要**显式** `as` 或 `From`/`Into`（若已实现）。

### 用户定义转换 trait

| Trait | 语义 | 返回值 |
|-------|------|--------|
| **`From<T>` / `Into<U>`** | 必然成功 | 直接得到目标类型 |
| **`TryFrom<T>` / `TryInto<U>`** | 可能失败 | `Result<_, Error>` |

### 解引用强制转换（Deref Coercion）

- 实现 **`Deref` / `DerefMut`** 的智能指针类型上，编译器可把 `&SmartPtr` **自动**转为 `&Target`（见 Book [15.2](../../Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) · [15.2.1 嵌套/坑点](../../Book/15-smart-pointers/15.2.1-Deref嵌套可变与编译坑.md)）。

---

## 2. 逻辑脉络与知识点关联

### `From` / `Into` 的非对称 blanket impl

- 逻辑上对称，但为避免循环 impl 冲突，标准库约定：
  - 你实现 **`From<T> for U`** → 自动获得 **`Into<U> for T`**
  - **不要**再手写 `Into`（除非特殊场景）

### 自反 impl 与 API 人体工学

```rust
impl<T> From<T> for T { ... }  // 标准库已有
```

- 泛型边界写 **`T: Into<MyType>`** 时：
  - 可传入能转成 `MyType` 的类型；
  - 也可**直接**传入 `MyType`（自反 `From`）。

---

## 3. 重点结论与实用要点

1. **实现只写 `From`，边界只写 `Into`**——经典准则。
2. **可能失败 → `TryFrom` / `TryInto`**——用 `Result`，别 `unwrap` 掩盖截断/解析错误。
3. **优先 `From` / `.into()`，慎用 `as`**——`as` 允许**有损**转换；除非 FFI / 底层语义明确需要。
4. **数值「变窄」**——`as` 能静默截断；安全代码用 `TryFrom` 或 Clippy 查 `cast_possible_truncation`。

---

## 4. 案例与代码要点

### `as`（有损）vs `Into`（trait 安全）

```rust
let x: u32 = 65_536;

let y = x as u16;        // ✅ 编译通过，高位截断（危险）
let y: u16 = x.into();   // ❌ u16 未 impl From<u32>，编译期拦截
```

### 泛型 `Into` 让调用端更顺

```rust
struct IanaAllocated(pub u64);

impl From<u64> for IanaAllocated {
    fn from(v: u64) -> Self { IanaAllocated(v) }
}

// fn is_iana_reserved(s: IanaAllocated)  // 不能直接传 42

fn is_iana_reserved<T: Into<IanaAllocated>>(s: T) {
    let s = s.into();
    // ...
}

is_iana_reserved(42);              // ✅
is_iana_reserved(IanaAllocated(0)); // ✅ 自反 From
```

### `TryFrom`（示意）

```rust
// u16::try_from(large_u32) -> Result<u16, TryFromIntError>
```

---

## 5. 易错细节

### 滥用 `as`

- 整数**截断**、符号变化等不会报错，只静默改位模式。
- 建议：Clippy 开启 `clippy::cast_possible_truncation` 等。

### 其它常见 Coercion（编译器静默）

| 转换 | 说明 |
|------|------|
| `&mut T` → `&T` | 可变借用降级为共享 |
| 引用 → 裸指针 | `&T` → `*const T` 等 |
| 无捕获闭包 → `fn` | 仅无环境捕获时 |
| `[T; N]` → `&[T]` | 数组退化为切片 |
| 具体类型 → `&dyn Trait` | trait 对象胖指针 |

需知道「哪里会自动转」，避免误以为 Rust 完全没有隐式转换。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 05](../ER-拓展索引.md#item-05)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 数值 | 无隐式转换，必须显式 |
| 实现 | 只写 `From` |
| 边界 | 用 `Into` |
| 可能失败 | `TryFrom` + `Result` |
| `as` | 可有损；优先 trait 转换 |
| Coercion | Deref、切片、trait 对象等少数自动 |
