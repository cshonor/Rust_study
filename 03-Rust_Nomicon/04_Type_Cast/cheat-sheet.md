# 速记 · Type Conversions · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **Coercion 自动弱化类型；Trait 匹配不自动转，receiver 除外。**
2. **点运算符：按值 → auto-ref → Deref 链 → unsizing 找方法。**
3. **`as` 比 coercion 宽但非传递；transmute 只验 size，&T→&mut T 永远 UB。**

## 自测

- [ ] 能举例 `&String` → `&str`、`&T` → `&dyn Trait` 的 coercion
- [ ] 能复述 `value.foo()` 的四步查找顺序
- [ ] 能解释 `e as U1 as U2` 合法为何 `e as U2` 可能非法
- [ ] 能说明 transmute 与 `as` 的本质区别（size vs 语义）
- [ ] 能对照 [coercions.rs](./src/coercions.rs) / [casts.rs](./src/casts.rs) 指出各转换类型

## 术语表

| 术语 | 含义 |
|------|------|
| coercion | 编译器自动、通常无害的类型弱化 |
| unsizing | 定长数组/Trait 对象等「变胖」的 coercion |
| cast (`as`) | 显式转换，范围大于 coercion，可能截断 |
| transmute | 同尺寸比特 reinterpret，几乎无防护 |
| 非传递性 | 分步 cast 合法 ≠ 一步 cast 合法 |
