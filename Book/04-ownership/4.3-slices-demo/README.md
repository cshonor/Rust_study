# 4.3 切片 demo — 编译对比

笔记：[4.3-切片slice.md](../4.3-切片slice.md)

## 运行主 demo

```bash
cargo run
cargo test
```

## `usize` 悬空索引 vs `&str` 借用（不用 playground，本地即可）

### 1. 切片方案：`clear` 编译失败（推荐先跑）

```bash
cargo check --example slice_blocks_clear
```

应看到类似：`cannot borrow ... as mutable because it is also borrowed as immutable`。

### 2. `usize` 方案：`clear` 能编译，切片才 panic

```bash
cargo test dangling_usize_panics_at_runtime
```

演示：`idx = 5` 在 `s.clear()` 后仍「存在」，再 `&s[0..idx]` 运行时越界。
