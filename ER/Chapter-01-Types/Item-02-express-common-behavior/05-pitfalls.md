# Item 2 · 易错细节

← [Item 2 目录](./README.md)

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

### 闭包 `Fn*` 易错

| 坑 | 说明 |
|----|------|
| `F: FnOnce()` 参数 | 调用会**消耗**闭包，通常不能传完再用 |
| API 写死 `Fn` | 过窄；优先 `FnOnce` 给调用方留空间 |
| `move \|\| ...` | 常为 `FnOnce`；可能移走外部变量导致后续不可用 |

→ 兼容规则与示例 → [02-logic-flow.md](./02-logic-flow.md)

### 静/动态分发易混

| 误区 | 正解 |
|------|------|
| vtable 里有方法代码 | 只有**地址**；实体代码在代码段 |
| `impl` 了 trait 就一定有 vtable | 只有用到 `dyn Trait` 才生成 vtable |
| 静态 / 动态二选一 | 同一类型可**并存**两路 → [Item 12 §07](../../Chapter-02-Traits/Item-12-generics-vs-trait-objects/06-dispatch-beginner-guide.md) |

---
