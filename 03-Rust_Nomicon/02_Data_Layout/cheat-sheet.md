# 速记 · Data Layout · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **`repr(Rust)` 默认可重排字段、插 padding；不要假设字段顺序。**
2. **DST 只能跟胖指针；ZST 偏移是 no-op、alloc(0) 危险。**
3. **跨 C/固定二进制格式 → `repr(C)`；勿滥用 `packed`。**

## 自测

- [ ] 能解释 `Option<&T>` 为何与 `&T` 同尺寸
- [ ] 能说出 ZST 在 MyVec 中的特殊处理（见 ch08）
- [ ] 能对照 [repr_rust.rs](./src/repr_rust.rs) 读出 `repr(C)` 与默认布局 size 差异

## 术语表

| 术语 | 含义 |
|------|------|
| niche | 利用无效位模式省 tag（如 null = None） |
| DST | 编译期未知 size 的类型 |
| ZST | size = 0 的类型 |
| padding | 对齐填充字节 |
