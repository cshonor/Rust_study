# trait-bounds-demo

RFR 第 2 章 · [08 Trait 限定](../08-trait-bounds.md) 配套示例。

```bash
cargo run
```

演示：`impl Display` 静态分发、`T: Display` 同类型双参数、`&dyn Error`、HRTB `for<'a> Fn(&'a str)`。
