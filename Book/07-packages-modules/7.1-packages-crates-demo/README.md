# 7.1 包与 Crate demo

笔记：[7.1](../7.1-包和crate.md) · [7.1.1](../7.1.1-二进制与库crate.md) · [7.1.2](../7.1.2-main调用分文件模块.md)

---

## 一、Package 三种布局（7.1）

| 目录 | 结构 | 命令 |
|------|------|------|
| `only_bin/` | 仅 `main.rs` | `cargo run` |
| `only_lib/` | 仅 `lib.rs` | `cargo build` |
| `bin_plus_lib/` | `lib.rs` + `main.rs` + `math/` | `cargo run` |
| `multi_bin/` | `lib.rs` + `src/bin/*.rs` | `cargo run --bin cli1` |

---

## 二、`pub` 与跨项目依赖（7.1.1）

| 目录 | 说明 | 命令 |
|------|------|------|
| `user_demo/` | `path` 依赖 `only_lib` | `cargo run` → 30 |
| `compile_fail/private_mod/` | `mod math;` 无 `pub` | `cargo check` → E0603 |

---

## 三、main 调 `a.rs`（7.1.2）

| 目录 | 结构 | 命令 |
|------|------|------|
| `via_lib/` | 有 lib：`lib.rs` 中转导出 | `cargo run` |
| `via_main_only/` | 无 lib：`main.rs` 里 `mod a` | `cargo run` |

---

## 快速跑通

```bash
# 布局
cd only_bin && cargo run
cd ../bin_plus_lib && cargo run

# main 调 a.rs
cd ../via_lib && cargo run
cd ../via_main_only && cargo run

# 反面
cd ../bin_plus_lib/compile_fail/private_mod && cargo check
```
