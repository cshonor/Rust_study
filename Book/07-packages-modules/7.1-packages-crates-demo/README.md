# 7.1 包与 Crate demo

笔记：[7.1-包和crate.md](../7.1-包和crate.md) · [7.1.1 二进制与库 crate](../7.1.1-二进制与库crate.md)

| 目录 | 结构 | 运行 |
|------|------|------|
| `only_bin/` | 仅 bin crate | `cargo run` |
| `only_lib/` | 仅 lib crate | `cargo build`（无 main，不能 run） |
| `bin_plus_lib/` | lib + bin + `math/` 子模块 | `cargo run` |
| `multi_bin/` | lib + 多个 `src/bin/*.rs` | `cargo run --bin cli1` |

```bash
cd only_bin && cargo run
cd ../only_lib && cargo build
cd ../bin_plus_lib && cargo run
cd ../multi_bin && cargo run --bin cli2
```

### compile_fail：`pub mod` 去掉 pub

```bash
cd bin_plus_lib/compile_fail/private_mod
cargo check   # 预期 E0603: module `math` is private
```
