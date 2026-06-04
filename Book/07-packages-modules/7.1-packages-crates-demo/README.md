# 7.1 包与 Crate demo

笔记：[7.1](../7.1-包和crate.md) · [7.1.1](../7.1.1-二进制与库crate.md)

| 目录 | 场景 | 命令 |
|------|------|------|
| `only_bin/` | ① 仅 bin crate | `cargo run` |
| `only_lib/` | ② 仅 lib crate | `cargo build` |
| `user_demo/` | ② 外部 `path` 依赖 `only_lib` | `cargo run` → 30 |
| `bin_plus_lib/` | ③ lib + bin + `math/` | `cargo run` → 3 |
| `multi_bin/` | 多个 `src/bin/*.rs` | `cargo run --bin cli1` |

```bash
cd only_bin && cargo run
cd ../only_lib && cargo build
cd ../user_demo && cargo run
cd ../bin_plus_lib && cargo run
```

### compile_fail

```bash
cd bin_plus_lib/compile_fail/private_mod && cargo check   # E0603
```
