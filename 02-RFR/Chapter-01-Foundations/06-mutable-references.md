# 3.2 Mutable References（可变引用 `&mut T`）

> 所属：**Borrowing and Lifetimes** · [← 章索引](./README.md)

## 语义

- **独占**借用：同一时刻，该内存上不得存在其它与之冲突的引用（`&` 或另一个 `&mut`）。
- 保证一次只有一条合法突变路径——借用检查器的核心之一。

## 与优化器

`&mut T` 携带 **no-aliasing** 假设：

- 编译器可重排、向量化、消除冗余 load/store（在内存模型允许范围内）。
- 这也是 Rust 能接近 C 性能、又保留安全抽象的原因之一。

## 与共享引用的对比

| | `&T` | `&mut T` |
|---|------|----------|
| 别名 | 多个共享 | 独占 |
| 突变 | 默认禁止 | 允许 |
| 优化假设 | 无突变 | 无其它别名 |

与 [05 共享引用](./05-shared-references.md) 成对阅读。

Book → [4.2 引用与借用](../../00-Book/04-ownership/4.2-引用与借用.md) · ER → [Item 15 借用检查器](../../01-ER/Chapter-03-Concepts/Item-15-borrow-checker/README.md)
