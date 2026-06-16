# 速记 · Safe & Unsafe · 自测

← [本章目录](./README.md) · 上一节：[06-hft-practice.md](./06-hft-practice.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **Safe = 编译器兜底；Unsafe = 程序员兜底，语法相同、多 5 种能力。**
2. **`unsafe fn/trait` 立契约；`unsafe {}` / `unsafe impl` 兑承诺。**
3. **Safe 无条件信 Unsafe；Unsafe 不能信 Safe 用户；安全是非局部的 → privacy 封装。**

## 5 种能力（仅 unsafe 块内）

1. 解引用 raw pointer（**创建**指针本身 Safe；解引用/偏移/转引用须 unsafe）
2. 调用 unsafe fn（含 FFI / intrinsic）
3. `unsafe impl` unsafe trait
4. 读/写 `static mut`
5. 读/写 `union` 字段

## 自测

- [ ] 能解释 `Vec::push` 为何对外是 Safe fn
- [ ] 能说明 `unsafe fn` 空函数体为何调用仍需 `unsafe {}`
- [ ] 能举例说明「Safe 改一行 → 全库 unsound」
- [ ] 能列出 HFT 场景下 unsafe 的典型用途（FFI / DPDK / 自定义分配器）
- [ ] 能区分「创建裸指针 Safe」与「解引用须 unsafe」
- [ ] 能对照 [raw_pointers.rs](./src/raw_pointers.rs) 说明 `*const` vs `*mut`

## 术语表（本章）

| 术语 | 含义 |
|------|------|
| UB | 未定义行为；编译器可任意假设未发生 |
| unsound | 库在 Safe 接口下仍可能 UB |
| invariant | unsafe 层维护的全局不变量（如 Vec len/cap/ptr） |
| 裸指针 | 无生命周期/借用的地址；解引用须 unsafe |
