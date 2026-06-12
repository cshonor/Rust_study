# Item 1 · 07 易错细节

← [Item 1 目录](./README.md)

## `Option` 与「空集合」

| 表达 | 何时用 |
|------|--------|
| `Vec<T>` / `String` 为空 | 通常已表示「没有内容」 |
| `Option<Vec<T>>` | 必须区分 **未提供** vs **提供了空集合** 时才嵌套 |

---

## 公开 `enum` 加变体 = 破坏性变更

- 外部 `match` 因**穷尽性**未覆盖新变体 → **编译失败**。
- 仅需 C 风格常量扩展时，可考虑 `#[non_exhaustive]`（调用方须留 `_` 等兜底分支）。

## 相关

- enum 基础 → [04-enum-adt.md](./04-enum-adt.md)
- Option 原则 → [05-option-result.md](./05-option-result.md)
