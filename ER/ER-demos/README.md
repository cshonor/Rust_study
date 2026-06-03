# Effective Rust — Demo 索引

与 The Book 的 `Book/**/*-demo/` 并列；此处 demo 聚焦 **ER 35 Items** 中的可运行示例。

## 运行

```bash
cd ER/ER-demos
cargo run -p item-15-borrow-checker
cargo test --workspace
```

## Item → Demo / Book 对照

| Item | ER demo | The Book demo（已有） |
|------|---------|----------------------|
| 1–2 | — | [6.1 enums](../../Book/06-enums-pattern-matching/6.1-enums-demo/) |
| 3–4 | — | [9.2 result](../../Book/09-error-handling/9.2-result-demo/) |
| 5 | — | [8.2 string](../../Book/08-collections/8.2-string-demo/) |
| 6 | — | [19.3 advanced types](../../Book/19-advanced-features/19.3-advanced-types-demo/) |
| 8 | — | [15.x smart pointers](../../Book/15-smart-pointers/) |
| 9 | — | [13.2 iterators](../../Book/13-iterators-closures/13.2-iterators-demo/) |
| 10–13 | — | [10.2 traits](../../Book/10-generics-traits-lifetimes/10.2-traits-demo/) |
| 14–15 | [item-15-borrow-checker](./item-15-borrow-checker/) | [10.3 lifetimes](../../Book/10-generics-traits-lifetimes/10.3-lifetimes-demo/) |
| 16 | — | [19.1 unsafe](../../Book/19-advanced-features/19.1-unsafe-rust-demo/) |
| 17 | — | [16.3 mutex-arc](../../Book/16-fearless-concurrency/16.3-mutex-arc-demo/) |
| 18 | [item-18-dont-panic](./item-18-dont-panic/) | [9.1 panic](../../Book/09-error-handling/9.1-panic-demo/) |
| 19 | — | [19.3 advanced types](../../Book/19-advanced-features/19.3-advanced-types-demo/) |
| 20 | [item-20-tlv](./item-20-tlv/) | — |
| 21–26 | [item-22-visibility](./item-22-visibility/)（22） | [7.2 modules](../../Book/07-packages-modules/7.2-modules-privacy-demo/) |
| 27 | — | [14.2 publish](../../Book/14-cargo-crates/14.2-crates-io-publish-demo/) |
| 28 | — | [19.5 macros](../../Book/19-advanced-features/19.5-macros-demo/) |
| 29–31 | — | 见 [Item 31](../Chapter-05-Tooling/Item-31-tooling-ecosystem.md) 工具表 |
| 30 | [item-30-black-box](./item-30-black-box/) | [11.1 tests](../../Book/11-testing/11.1-tests-demo/) |
| 32 | [`.github/workflows/er-study-ci.yml`](../../.github/workflows/er-study-ci.yml) | — |
| 33 | [item-33-no-std](./item-33-no-std/) | — |
| 34–35 | [item-34-ffi-box](./item-34-ffi-box/) | [19.1 unsafe](../../Book/19-advanced-features/19.1-unsafe-rust-demo/) |

拓展笔记全文：[ER-拓展索引.md](../ER-拓展索引.md)
