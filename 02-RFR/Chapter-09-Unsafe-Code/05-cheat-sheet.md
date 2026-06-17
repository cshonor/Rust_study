# 3.1 · 什么会出错 · 速记

← [05 What Can Go Wrong](./05-what-can-go-wrong.md) · [章索引](./README.md)

---

## UB 三特征

1. **静默** — 不崩溃也错  
2. **版本** — 升级编译器暴露  
3. **优化畸变** — 契约破坏 → LLVM 乱优化  

## 三大契约

| 契约 | 一句话 |
|------|--------|
| **Validity** | 已初始化 · 对齐 · 不越界 · 不悬垂 |
| **Aliasing** | 多读 XOR 单写 |
| **Layout** | 类型布局不可乱转、乱改 |

## 误区

- `unsafe` **不**消除 UB  
- 没崩溃 **≠** 没 UB  
- Panic **≠** UB  

## 分项

`06` validity · `07` panic · `08` cast · `09` drop

## 工程

最小 unsafe · `// SAFETY:` · Miri
