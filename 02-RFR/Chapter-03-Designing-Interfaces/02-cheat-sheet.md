# 1.2 · 通用 Trait · 速记

← [02 Common Traits](./02-common-traits-for-types.md) · [章索引](./README.md)

---

## 适用谁

库作者最佳实践 · 内部业务灵活 · `Strategy` 等自定义 trait 按需 · **impl std trait 仍守三类**

## 划分标准

**影响范围 · 副作用 · 违反代价**

| 类 | Trait | 策略 |
|----|-------|------|
| Ⅰ 几乎总有 | `Debug` · `PartialEq` / `Eq` | 默认 derive |
| Ⅱ 线程契约 | `Send` · `Sync` | 多数自动；例外文档 |
| Ⅲ 谨慎 | `Copy` · `Hash` | 按需 · 成对 · 小值才 Copy |

## derive 与风险

`Debug`/`Eq` 放心 derive · `Copy` 须显式 · `Hash`+`Eq` 同写

→ 详例：[02-1 完整解读](./02-1-common-traits-full-guide.md)

## 自测

- [ ] 三类各自「违反代价」是什么？  
- [ ] 带 `String` 的订单能 `Copy` 吗？
