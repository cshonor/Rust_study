# Item 1 · 06 设计模式与重构

← [Item 1 目录](../README.md)

Effective Rust Item 1 的核心主张：**用类型表达设计意图，让无效状态无法被类型表达**（*Make invalid states inexpressible*）。

本章不是讲 GoF 那套「23 种设计模式」，而是讲 **Rust 里用类型系统做设计**——不靠类和继承，靠 struct / enum / trait 在编译期把非法组合挡在外面。

## 重点结论

1. **用类型表达设计意图**——对人、对编译器都清晰。
2. **Make invalid states inexpressible**——合法组合才写得出来；非法组合**根本写不进类型**。
3. **缺失值 / 失败**——分别用 `Option` / `Result`，见 [05-option-result.md](./05-option-result.md)。

---

## 「让无效状态无法被类型表达」是什么意思？

一句话：**把「只能处于其中一种状态」这件事写进类型，而不是靠注释、运行时检查或 `Option` 字段凑出来。**

### 高频交易订单：struct 反模式

用「一个大 struct + 一堆可选字段」存订单，很容易留下**逻辑上不可能同时成立**的组合：

```rust
struct Order {
    create_time: Option<u64>,   // 挂单时间
    trade_time: Option<u64>,    // 成交时间
    price: Option<i64>,         // 成交价格
    cancel_time: Option<u64>,   // 撤单时间
}
```

问题在于：

| 实际业务 | struct 却允许 |
|----------|---------------|
| 未成交订单只有 `create_time` | `create_time` 和 `trade_time` 同时 `Some` |
| 已成交订单有 `trade_time` + `price` | 已成交还带 `create_time` |
| 已撤单只有 `cancel_time` | 撤单了还留着 `price` |

编译器**不会**因为你填了矛盾字段而报错——只能靠测试、Code Review 或线上 bug 去发现。对容不得错的高频交易系统，这种 bug 代价极高。

### 同一业务：用 enum 从根上消灭

```rust
enum OrderStatus {
    Pending { create_time: u64 },
    Filled { trade_time: u64, price: i64 },
    Cancelled { cancel_time: u64 },
}
```

每个变体**只携带该状态下合法的字段**：

- `Pending` → 必有 `create_time`，不可能带 `trade_time` / `price`
- `Filled` → 必有 `trade_time` 和 `price`，不可能还带挂单语义
- `Cancelled` → 必有 `cancel_time`

`match order_status { ... }` 时编译器要求**穷尽所有变体**，新增状态必须改 enum，漏分支会编译失败——这就是「无效状态无法被类型表达」。

> 与 [04-enum-adt.md](./04-enum-adt.md) 里的 `OrderState` 示例同一思路；这里强调**为什么要从 struct 迁到 enum**。

---

## 反模式 → 用 enum 消灭非法组合

**重构前**（靠注释约定，字段可处于非法组合）：

```rust
struct ScreenColor {
    monochrome: bool,
    fg_color: RgbColor, // 注释：monochrome 为 true 时 fg 应为 0
}
```

**重构后**（无效组合在类型层面不存在）：

```rust
enum Color {
    Monochrome,
    Foreground(RgbColor),
}
struct ScreenColor {
    color: Color,
}
```

---

## 布尔参数 → 具名枚举

```rust
// ❌ 易填错、顺序可颠倒而不报错
// print_page(true, false);

// ✅ 意图清晰，错序/错值更易被类型拦住
enum Sides { One, Both }
enum Output { BlackAndWhite, Color }
fn print_page(sides: Sides, output: Output) { /* ... */ }
```

## 相关

- ADT 语法 → [04-enum-adt.md](./04-enum-adt.md)
- 易错细节 → [07-pitfalls.md](./07-pitfalls.md)
