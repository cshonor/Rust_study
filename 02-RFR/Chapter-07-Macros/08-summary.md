# 3. Summary（小结）

> [← 章索引](./README.md)

## 四句话带走

1. **能泛型则泛型** — 宏是语法形状与样板的后备，不是默认方案。  
2. **声明宏** = TokenTree 上的模式匹配 + 转录；注意卫生与 `$crate`。  
3. **过程宏** = `TokenStream` 函数；derive / 属性 / 类函数三形态；管 **Span** 与**编译代价**。  
4. 排错靠 **`cargo-expand`** 看展开，不靠猜。

## 下一章

→ [第 8 章 Async](../Chapter-08-Async/README.md)

## 本章笔记索引

| # | 文件 |
|---|------|
| 01–03 | [声明宏](./01-when-to-use-declarative-macros.md) · [02](./02-how-declarative-macros-work.md) · [03](./03-how-to-write-declarative-macros.md) |
| 04–07 | [过程宏](./04-types-of-procedural-macros.md) · [05](./05-cost-of-procedural-macros.md) · [06](./06-so-you-think-you-want-a-macro.md) · [07](./07-how-procedural-macros-work.md) |
| 08 | [小结](./08-summary.md) |
