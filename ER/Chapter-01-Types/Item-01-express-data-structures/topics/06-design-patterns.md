# Item 1 · 06 设计模式与重构

← [Item 1 目录](../README.md)

Effective Rust Item 1 的核心主张：**用类型表达设计意图，让无效状态无法被类型表达**。

## 重点结论

1. **用类型表达设计意图**——对人、对编译器都清晰。
2. **Make invalid states inexpressible**——合法组合才写得出来。
3. **缺失值 / 失败**——分别用 `Option` / `Result`，见 [05-option-result.md](./05-option-result.md)。

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
