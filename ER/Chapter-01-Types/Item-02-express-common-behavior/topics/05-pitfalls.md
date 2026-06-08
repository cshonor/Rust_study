# Item 2 · 易错细节

← [Item 2 目录](../README.md)

### C++ 模板 vs Rust 泛型

| | C++ 模板 | Rust 泛型 |
|--|----------|-----------|
| 约束 | 往往隐式（有同名方法就能用） | **必须显式 Trait bound** |
| 风险 | 「鸭子类型」误用可能很晚才暴露 | 契约在签名处拦截 |

### Trait 对象的对象安全（Object Safety）

并非所有 trait 都能写成 `dyn Trait` / `&dyn Trait`。常见**不能**对象化的情况：

- 方法返回 `Self`；
- 方法带未消去的泛型参数。

→ 编译器无法建稳定 vtable，**拒绝**生成 trait 对象。

---
