# 速记 · no_std · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **`libc` 必须 `default-features = false`，否则隐式拉回 `std`。**
2. **裸机 exe 用 `#![no_main]` + 手动 `_start`；常需 nightly lang items 与 `compiler_builtins`。**
3. **`#[panic_handler] fn(&PanicInfo) -> !` 全局唯一；dev 用 semihosting，release 用 halt。**

## 自测

- [ ] 能解释为何 `default-features = true` 的 libc 破坏 no_std
- [ ] 能列出 `no_main` 与默认 `main` 的区别
- [ ] 能写出 `panic_handler` 的正确签名
- [ ] 能说明 stable no_std 库 vs nightly 裸机 exe 的差异
- [ ] 能对照 [src/lib.rs](./src/lib.rs) 找到 panic 实现

## 术语表（本章）

| 术语 | 含义 |
|------|------|
| no_std | 不链接标准库；仍可用 core/alloc |
| lang item | 编译器/runtime 期望的符号（如 eh_personality） |
| panic_handler | no_std 下 panic 的唯一处理入口 |
| semihosting | 调试时通过主机 I/O 输出 panic |

## 构建速查

```bash
cd 03-Rust_Nomicon/10_NoStd
cargo run                              # std 宿主 demo
cargo build --no-default-features      # 纯 no_std 库
cargo test                             # std 下单元测试
```
