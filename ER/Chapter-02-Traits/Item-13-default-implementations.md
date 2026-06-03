# Item 13: Use default implementations to minimize required trait methods

> **Effective Rust** · [Chapter 2 — Traits](../ER-本书目录.md)  
> **中文**：用默认实现最小化 trait 中必需的方法  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo / 代码练习

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| trait 定义、默认方法 | [10.2 trait](../../Book/10-generics-traits-lifetimes/10.2-trait.md) |
| `Iterator` / `next` | [13.2 迭代器](../../Book/13-iterators-closures/13.2-使用迭代器处理元素序列.md) |
| 对象安全、`Sized` | [Item 12](./Item-12-generics-vs-trait-objects.md) |
| 迭代器适配器 | [Item 9](../Chapter-01-Types/Item-09-iterator-transforms.md) |

---

## 1. 核心知识点与关键定义

### 两类受众的张力

| 受众 | 诉求 |
|------|------|
| **实现者 (Implementors)** | 尽量少写方法 |
| **使用者 (Users)** | 方法越多越好、越方便 |

### 默认实现（Default Implementations）

- 在 **trait 定义里**直接写方法体，基于更「基元」的操作组合出高阶 API。
- 实现类型**只需**实现少量必需项；其余自动继承默认逻辑。

### 方法上的 Trait Bound

- 默认方法可带 **`where`**：仅当类型满足额外契约时，该方法才对调用方**可见/可用**。

---

## 2. 逻辑脉络

```text
强制少量基元方法（如 next / len）
  → trait 内默认实现派生大量便捷 API
  → 实现者省力、使用者功能全
  → 库演进：新增带默认体的方法 ≈ 向后兼容
  → 具体类型可 override 更高效实现
```

### 向后兼容

- 已发布 trait **新增带 default body 的方法** → 旧 impl **不必改**，通常 **不破坏** API。
- 对比：新增**无默认**的必需方法 → 所有 impl 必须补实现 → **破坏性**。

### 可覆盖（Override）

- 默认实现是**后备**；若类型有 O(1) 捷径（如 `is_empty`），可 **override** 默认的 O(n) 逻辑。

---

## 3. 重点结论与实用要点

1. **小强制面 + 大可用面**——学标准库：`Iterator` 只强制 `next()`，赠送 50+ 方法。
2. **`where` 做条件增强**——如 `cloned()` 仅在 `T: Clone` 时可用，不增加基元 impl 负担。
3. **公共库 API 演进**——优先用**默认实现**扩展 trait，而非逼所有下游改 impl。
4. 与 **Item 12**：默认方法若含泛型 / `Self`，用 **`where Self: Sized`** 保护 **对象安全**（见 §6）。

---

## 4. 案例与代码要点

### `ExactSizeIterator::is_empty`

```rust
trait ExactSizeIterator: Iterator {
    fn len(&self) -> usize; // 必需

    fn is_empty(&self) -> bool {
        self.len() == 0     // 默认实现
    }
}
```

### `Iterator`：只写 `next()`

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // 唯一必需

    // map, filter, fold, collect, ... 均有 default
}
```

### 带 bound 的默认方法：`cloned`

```rust
fn cloned<'a, T>(self) -> Cloned<Self>
where
    T: 'a + Clone,
    Self: Sized + Iterator<Item = &'a T>,
{
    Cloned::new(self)
}
```

仅当 `Item` 为 `&T` 且 `T: Clone` 时，调用方才能用 `.cloned()`。

---

## 5. 易错细节

### 默认 trait 方法 vs 固有方法同名

- trait **新增**带 default 的方法名，若与类型 **inherent impl** 同名：
  - 普通调用 **`obj.method()`** → **固有方法优先**（遮蔽 trait 默认）。
  - 要调 trait 版本：**完全限定语法**  
    `<Concrete as Trait>::method(&obj)`

→ 演进 public trait 时注意命名，避免与常见 inherent 方法冲突。

---

## 6. 后续拓展（待补）

- [ ] 默认方法 + 泛型 / `Self` + `where Self: Sized` 与 `dyn Trait`（链 Item 12）
- [ ] Serde 等 crate：小核心 API + 大默认方法族

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 张力 | 实现者要少写，使用者要多用 |
| 模式 | 基元方法 + default 派生 |
| 演进 | 新 default 方法通常兼容 |
| override | 有更优算法就重写 |
| 冲突 | inherent 遮蔽 → UFCS |
