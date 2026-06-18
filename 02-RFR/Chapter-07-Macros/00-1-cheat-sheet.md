# 00.1 · Token 与宏展开 · 速记

← [00-1 Token 链路](./00-1-token-and-macro-pipeline.md) · [00 hub](./00-macros-overview.md) · [章索引](./README.md)

---

## Token

词法分析产物 · 整文件 = Token 流 · **不**来回变字符串

## 声明宏匹配谁？

`vec!` = 调用标记 · **`[1,2,3]`** = 参与匹配的参数 Token

## 声明宏本质

Token 进 → 模板替换 → Token 出（无「转回 .rs 再 lex」）

## 过程宏本质

`TokenStream` → 编译期 Rust 程序（常 syn/quote）→ `TokenStream`

## 编译顺序

lex → **宏展开** → parse AST → 类型检查 → 单态化 → LLVM

## 四误区

无字符串中转 · 不只括号外匹配 · 过程宏非升级版 · 类型检查在展开后

## 自测

- [ ] `vec![1,2,3]` 哪几个 Token 参与 `macro_rules!` 匹配？  
- [ ] `cargo expand` 展示的是编译器哪一步？  
- [ ] 过程宏边界 API 是 AST 还是 `TokenStream`？
