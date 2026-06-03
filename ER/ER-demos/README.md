# Effective Rust — Demo 索引

与 The Book 的 `Book/**/*-demo/` 并列；此处 demo 聚焦 **ER 35 Items** 中的可运行示例。

## 运行

```bash
cd ER/ER-demos
cargo test --workspace
cargo run -p item-03-option-result
cargo run -p item-04-error-types
cd item-24-re-export && cargo run -p consumer
```

## Item → Demo / Book 对照

| Item | ER demo | The Book demo（已有） |
|------|---------|----------------------|
| 1–2 | — | [6.1 enums](../../Book/06-enums-pattern-matching/6.1-enums-demo/) |
| 3 | [item-03-option-result](./item-03-option-result/) | [9.2 result](../../Book/09-error-handling/9.2-result-demo/) |
| 4 | [item-04-error-types](./item-04-error-types/) | [9.2 result](../../Book/09-error-handling/9.2-result-demo/) |
| 5 | — | [8.2 string](../../Book/08-collections/8.2-string-demo/) |
| 6 | — | [19.3 advanced types](../../Book/19-advanced-features/19.3-advanced-types-demo/) |
| 8–9 | — | [15.x](../../Book/15-smart-pointers/)、[13.2 iterators](../../Book/13-iterators-closures/13.2-iterators-demo/) |
| 10–13 | — | [10.2 traits](../../Book/10-generics-traits-lifetimes/10.2-traits-demo/) |
| 14–15 | [item-15-borrow-checker](./item-15-borrow-checker/) | [10.3 lifetimes](../../Book/10-generics-traits-lifetimes/10.3-lifetimes-demo/) |
| 16 | — | [19.1 unsafe](../../Book/19-advanced-features/19.1-unsafe-rust-demo/) |
| 17 | — | [16.3 mutex-arc](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| 18 | [item-18-dont-panic](./item-18-dont-panic/) | [9.1 panic](../../Book/09-error-handling/9.1-panic-demo/) |
| 19–20 | [item-20-tlv](./item-20-tlv/)（20） | [19.3 advanced types](../../Book/19-advanced-features/19.3-advanced-types-demo/) |
| 21 | — | [14.2 publish](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 22 | [item-22-visibility](./item-22-visibility/) | [7.2 modules](../../Book/07-packages-modules/7.2-定义模块来控制作用域与私有性.md) |
| 23 | — | [7.4 use](../../Book/07-packages-modules/7.4-使用use关键字将名称引入作用域.md) |
| 24 | [item-24-re-export](./item-24-re-export/) | — |
| 25–26 | [item-26-feature-creep](./item-26-feature-creep/)（26） | [14.3 workspace](../../Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| 27–31 | — | [14.2](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md)、[19.5 宏](../../Book/19-advanced-features/19.5-宏.md) |
| 30 | [item-30-black-box](./item-30-black-box/) | [11.1 tests](../../Book/11-testing/11.1-如何编写测试.md) |
| 32 | [`.github/workflows/er-study-ci.yml`](../../.github/workflows/er-study-ci.yml) | — |
| 33 | [item-33-no-std](./item-33-no-std/) | — |
| 34 | [item-34-ffi-box](./item-34-ffi-box/) | [19.1 unsafe](../../Book/19-advanced-features/19.1-unsafe-rust-demo/) |
| 35 | [item-35-bindgen](./item-35-bindgen/) | — |

拓展笔记全文：[ER-拓展索引.md](../ER-拓展索引.md)
