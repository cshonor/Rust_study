# 2.1 Types of Procedural Macros（过程宏类型）

> 所属：**Procedural Macros** · [← 章索引](./README.md)

过程宏 = **Rust 函数**：入参 **`TokenStream`**，出参 **`TokenStream`**（常配合 **`syn`** / **`quote`**）。

## 三类

| 种类 | 形态 | 典型用途 |
|------|------|----------|
| **派生 (derive)** | `#[derive(Foo)]` | 为类型**追加** `impl` 等（如 `Serialize`、`Clone`） |
| **类函数 (function-like)** | `foo!(...)` | 编译期变换、DSL、字面量求值 |
| **属性 (attribute)** | `#[bar(...)]` 修饰项 | 路由、测试封装、字段变换（如 pin 类生成） |

**derive 注意**：一般**追加**代码，不改变用户手写项的主体文本；可与辅助属性配合。

## Span 与卫生（预览）

过程宏的「卫生」**不自动**等同声明宏；常需 **`Span`** 控制标识符解析位置 → [07 如何工作](./07-how-procedural-macros-work.md)
