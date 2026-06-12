# 1.1 Naming Practices（命名惯例）

> 所属：**Unsurprising** · [← 章索引](./README.md)

优秀接口应符合社区直觉：用户「盲猜」用法时也应大概率猜对。

## 原则

- **复用约定俗成的名字与语义**，不要发明与 std / 生态冲突的新含义。
- 方法名应反映**是否消耗 `self`**、**返回借用还是拥有值**。

## 常见对照

| 名字 | 社区预期 |
|------|----------|
| `iter` / `iter_mut` | **借用**迭代器，不消耗容器 |
| `into_iter` | **消耗**容器，产出 owned 项 |
| `as_ref` / `as_mut` |  cheap 借用视图 |
| `into_inner` / `into_parts` | **消耗**包装类型，取出内部 |
| `try_*` | 返回 `Result` 或 `Option` |
| `with_*` | 配置/builder 风格，常消耗或可变 |

## 反例

- `iter()` 却消耗所有权 → 破坏预期。
- `get()` 与 `get_mut()` 不对称且无文档说明 → 增加挫败感。

Book → [7.2 引用项命名](../../00-Book/07-packages-modules/7.2-引用项命名.md)
