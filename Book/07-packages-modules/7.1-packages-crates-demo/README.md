# 7.1 包与 Crate demo

笔记：[7.1](../7.1-包和crate.md) · [7.1.1](../7.1.1-二进制与库crate.md) · [7.1.2](../7.1.2-main调用分文件模块.md)

| 目录 | 场景 | 命令 |
|------|------|------|
| `only_bin/` | 仅 bin crate | `cargo run` |
| `only_lib/` | 仅 lib crate | `cargo build` |
| `user_demo/` | 外部 `path` 依赖库 | `cargo run` |
| `bin_plus_lib/` | lib + bin + `math/` | `cargo run` |
| `via_lib/` | **结构1** lib 中转导出 `a.rs` | `cargo run` |
| `via_main_only/` | **结构2** main 里 `mod a` | `cargo run` |
| `multi_bin/` | 多个 `src/bin/*.rs` | `cargo run --bin cli1` |

```bash
cd via_lib && cargo run
cd ../via_main_only && cargo run
```

### compile_fail

```bash
cd bin_plus_lib/compile_fail/private_mod && cargo check   # E0603
```
