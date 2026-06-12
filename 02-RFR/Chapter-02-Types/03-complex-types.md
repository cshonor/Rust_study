# 1.3 Complex Types（复合类型）

> 所属：**Types in Memory** · [← 章索引](./README.md)

结构体、枚举、元组、数组等在内存中并非「简单拼接比特」——编译器会做布局与优化决策。

## 结构体 (struct)

- 字段按 [02 布局](./02-layout.md) 规则排列；可含 padding。
- **字段重排**（默认 `repr(Rust)`）：减小 `size_of`，但破坏「源码顺序 = 内存顺序」直觉。

## 枚举 (enum)

- **标签 + 载荷**：判别式 (discriminant) 标识当前变体，载荷区存放对应数据。
- **Niche optimization**：利用「非法比特模式」编码 `Option<NonNull<T>>` 等，使 `size == sizeof(T)`。
- **空枚举** `enum Void {}`：无合法值；在类型理论中对应 uninhabited type。

## 元组与数组

| 类型 | 内存直觉 |
|------|----------|
| 元组 | 类似匿名 struct，按元素顺序布局 |
| `[T; N]` | 连续 N 个 `T`，`size = N * size_of::<T>()`（含对齐） |
| `Vec<T>` | 三元组 `(ptr, len, cap)` 在栈上；元素在堆 |

## 与 Trait 的衔接

复合类型是 **Trait impl 的载体**；`Copy`/`Clone`/`Drop` 等行为直接取决于其字段与布局 → 第 2 章后半 Trait 部分。

Book → [5.1 结构体](../../00-Book/05-structs/5.1-定义结构体.md) · [6.1 枚举](../../00-Book/06-enums-pattern-matching/6.1-定义枚举.md)
