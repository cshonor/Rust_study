# get_ · 速记

← [01-3](./01-3-get-series.md) · [01 命名 hub](./01-naming-practices.md)

---

## 口诀

`Option` 安全访问 · 越界 `None` 不 panic

## 对比

`v[5]` panic · `v.get(5)` → `None`

## 自测

- [ ] 外部输入索引该用 `get` 还是 `[]`？
