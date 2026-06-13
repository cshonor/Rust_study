# 1.2 How Declarative Macros Work（声明宏如何工作）

> 所属：**Declarative Macros** · [← 章索引](./README.md)

直觉：受约束的「**模式匹配 + 转录**」，发生在 **TokenTree** 层面 — **不是** C 预处理器的纯文本拼接。

## 展开管线

1. 调用 `foo!(...)` → 编译器收集 **token 流**
2. 与 **`macro_rules!`** 的**匹配器**比对
3. 匹配成功 → **转录器**生成新 token 流
4. 展开结果必须是**合法 Rust 片段**（表达式 / 项 / 类型等）

## TokenTree

- 输入被切成 **TokenTree**，**不必**已是完整合法 Rust 程序，但须是合法 token 流
- 因此比「任意文本替换」安全，但仍可能生成**语义错误**的合法语法

## 与过程宏对比（预览）

| | 声明宏 | 过程宏 |
|---|--------|--------|
| 定义 | `macro_rules!` | `#[proc_macro]` 等 Rust 函数 |
| 逻辑 | 模式 + 模板 | 任意 Rust 代码处理 `TokenStream` |

→ [07 过程宏如何工作](./07-how-procedural-macros-work.md)
