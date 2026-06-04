# 7.1 包与 Crate demo

笔记：[7.1-包和crate.md](../7.1-包和crate.md)

三种 Package 布局，各为**独立 Cargo 项目**：

| 目录 | 结构 | 运行 |
|------|------|------|
| `only_bin/` | 仅二进制 crate | `cargo run` |
| `bin_plus_lib/` | 库 + 主二进制 | `cargo run` |
| `multi_bin/` | 库 + 多个 `src/bin/*.rs` | `cargo run --bin cli1` / `--bin cli2` |

```bash
cd only_bin && cargo run
cd ../bin_plus_lib && cargo run
cd ../multi_bin && cargo run --bin cli2
```
