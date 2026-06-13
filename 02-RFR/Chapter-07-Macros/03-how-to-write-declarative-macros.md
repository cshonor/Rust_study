# 1.3 How to Write Declarative Macros（如何编写声明宏）

> 所属：**Declarative Macros** · [← 章索引](./README.md)

## 匹配器与转录器 (Matchers & Transcribers)

- **匹配器**：片段分类符（`:expr`、`:ident`、`:ty`、`:pat` …）+ 重复（`*` / `+` / `?`）
- **转录器**：捕获的**元变量**填入模板生成代码

```rust
macro_rules! twice {
    ($e:expr) => {
        ($e) + ($e)
    };
}
```

## 卫生 (Hygiene)

| | 行为 |
|---|------|
| **局部绑定**（`let` 等） | 通常**卫生** — 宏内名字不意外捕获调用点同名变量 |
| **项级生成**（`fn` / `mod` / `type`） | 进入正常模块系统，**调用方可见** — 否则无法「生成 API」 |

## `$crate`

宏展开中引用**定义该宏的 crate**，避免跨 crate 展开时路径漂移：

```rust
macro_rules! my_macro {
    () => {
        $crate::internal_helper()
    };
}
```

也可配合 `::core::...` 等显式根路径。

→ demo [19.5-macros-demo](../../00-Book/19-advanced-features/19.5-macros-demo/) · 展开 [cargo-expand](../Chapter-13-Rust-Ecosystem/01-tools.md)
