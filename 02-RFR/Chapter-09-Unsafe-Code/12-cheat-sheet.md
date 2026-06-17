# 4.3 · 验证 unsafe · 速记

← [12 Check Your Work](./12-check-your-work.md) · [11 文档](./11-documentation.md) · [章索引](./README.md)

---

## 四层验证（推荐顺序）

1. **单元/集成测试** — 边界、panic、生命周期、并发  
2. **Miri** — Rust UB 首选（nightly）  
3. **ASAN / Valgrind** — FFI、堆栈越界  
4. **人工评审** — 双人复核 unsafe  

## Miri 命令

```bash
rustup +nightly component add miri
cargo +nightly miri test
```

## Miri 擅长

越界 · UAF · 未初始化 · Validity · provenance · Stacked Borrows

## Miri 短板

无 C FFI · 慢 · 非真实分配器

## 工具分工

| Miri | ASAN | Valgrind |
|------|------|----------|
| Rust 语义 UB | 堆/栈/竞争/跨语言 | C 泄漏/ABI |

## 评审清单

`// SAFETY:` · `# Safety` · `# Invariants` · 生命周期/对齐/所有权/并发

## 自测

- [ ] 为何 release `cargo test` 不够，还须 Miri？  
- [ ] FFI 模块为何须 ASAN 而非只靠 Miri？  
- [ ] Miri 报错时如何借助 §11 文档定位契约？
