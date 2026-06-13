# layout-demo

RFR 第 2 章 · [02 布局](../02-layout.md) 配套可运行示例。

对比 `Test { a: u8, b: u32, c: u8 }`、`DefaultOrder { a: u8, b: u32, c: u16 }`、`enum Either`、`Option<&T>` niche 等的 `size_of` / `align_of` / `offset_of`。

```bash
# 在仓库根目录
cargo run --manifest-path 02-RFR/Chapter-02-Types/layout-demo/Cargo.toml
```

使用 **stable** Rust 即可（`offset_of!` 已稳定）。
