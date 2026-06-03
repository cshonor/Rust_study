# Item 24: 公开依赖 + `pub use rand`

`dep-lib` 在 API 里使用 `rand 0.8` 并 **重导出**；`consumer` 通过 `dep_lib::rand` 构造 RNG，类型一致。

## 错误用法（勿编译）

若 consumer 自己 `use rand`（且解析到 **另一 semver 版本**），传入 `pick_number_with` 会报 trait 不满足：

```text
the trait bound ThreadRng: rand_core::RngCore is not satisfied
```

## 诊断

```bash
cd ER/ER-demos/item-24-re-export
cargo tree -p consumer -d rand
```

## 运行

```bash
cargo run -p consumer
```
