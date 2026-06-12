# Item 24 · 记忆卡片

← [Item 24 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 公开依赖 | API 里出现外部类型 = 与其 Semver 绑定 |
| 多版本 | 同名不同版 = **不同类型** |
| 必须做 | API 用了就要 **`pub use`** |
| 下游 | 用 `your_crate::dep` 构造匹配类型 |
| 报错 | trait not satisfied → 先查**版本重复** |
| 谨慎 | 能不用外部类型在 API 里就别用 |
