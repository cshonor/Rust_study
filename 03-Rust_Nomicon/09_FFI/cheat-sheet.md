# 速记 · FFI · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **所有 C 调用都是 `unsafe`；对外暴露 Safe API 须在 Rust 侧包装边界。**
2. **跨边界 struct 用 `repr(C)`；字符串用 `CString`；`Option<fn ptr>` 的 None = null。**
3. **panic 不可跨界 → FFI 出口用 `catch_unwind`；opaque 用私有字段 struct + PhantomData。**

## 自测

- [ ] 能写出 `extern "C"` 声明与 `#[link]` 的基本形式
- [ ] 能说明 `cdylib` + `#[no_mangle]` 导出给 C 的步骤
- [ ] 能解释带状态回调为何需要 `*mut` + 生命周期管理
- [ ] 能说明 `static mut` 外部全局为何读写都 unsafe
- [ ] 能解释 panic 跨入 C 为何 UB
- [ ] 能对照 [src/call_c.rs](./src/call_c.rs) 与 [src/unwind.rs](./src/unwind.rs) 说出边界处理

## 术语表（本章）

| 术语 | 含义 |
|------|------|
| ABI / 调用约定 | `extern "C"` 等函数调用与符号约定 |
| niche | `Option<NonNull<T>>` 用 null 表示 None |
| opaque | C 侧不公开布局，Rust 侧仅持指针 |
| catch_unwind | 在 FFI 边界捕获 panic，防栈展开跨界 |

## 源码索引

| 文件 | 演示 |
|------|------|
| [src/call_c.rs](./src/call_c.rs) | 调用 C |
| [src/export_to_c.rs](./src/export_to_c.rs) | 导出给 C |
| [src/callbacks.rs](./src/callbacks.rs) | 回调 |
| [src/interop.rs](./src/interop.rs) | repr(C) / CString |
| [src/globals.rs](./src/globals.rs) | 外部全局 |
| [src/nullable.rs](./src/nullable.rs) | Option niche |
| [src/unwind.rs](./src/unwind.rs) | catch_unwind |
| [src/opaque.rs](./src/opaque.rs) | 不透明类型 |
