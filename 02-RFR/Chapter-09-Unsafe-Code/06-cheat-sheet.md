# 3.2 · Validity · 速记

← [06 Validity](./06-validity.md) · [05 三大契约](./05-what-can-go-wrong.md) · [章索引](./README.md)

---

## 一句

**比特模式是否符合类型的合法值** — 违反 → UB。

## 初始化 vs 有效

| | 已初始化 | Valid |
|---|:--------:|:-----:|
| 内存有比特 | ✅ | — |
| 比特合法 | — | ✅ |
| `0x02` 当 `bool` | ✅ | ❌ |

## 三类细则

| 类型 | 约束 |
|------|------|
| `&T` | 非空 · 对齐 · 合法对象 · 无悬垂 |
| `bool` | 仅 `0` / `1` |
| `enum` | tag 与载荷匹配 |

## 工程

`MaybeUninit` · 少 `transmute` · Miri · `// SAFETY:`

## 自测

- [ ] 为何 `assume_init` 要同时管「写过」和「位模式合法」？  
- [ ] 无效 `bool` 为何影响 LLVM 分支优化？  
- [ ] Panic 与 Validity 违反都是 UB 吗？
