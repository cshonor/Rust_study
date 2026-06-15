# 第 4 章 · 上下文相关分析 · 速记与自测

← [本章目录](./README.md) · 上一节：[04-syntax-directed-translation.md](./04-syntax-directed-translation.md)

---

## 本章速记

```text
§1  语法≠语义 · 声明/类型/维数等需上下文 · =语义分析
§2  类型系统：基本+合成 · 强/弱 · 静/动 · 安全+代码生成
§3  属性文法：合成(子→父) · 继承(父→子) · 工业少用纯AG
§4  SDT：嵌入Action · YACC $/$$ · 建AST · 符号表/作用域
前端收官：Token→结构→含义 → 进 ch5 IR
```

---

## 三句背诵

1. **ch4 答「有没有意义」：CFG 管形式，上下文相关管语义。**
2. **类型检查是核心；属性文法理论化，SDT 工程化。**
3. **合成向上、继承向下；工业用 Action 建 AST，不囤整棵分析树。**

---

## 与 CI / Rust 对照

| 橡书 ch4 | 本仓库 |
|----------|--------|
| 语义 / 作用域 | [jlox ch11 Resolver](../../../01_Crafting-Interpreters/part02_jlox/chapter11_resolving-and-binding/README.md) |
| AST | [jlox ch5](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/README.md) |
| 动态类型 | [jlox ch7](../../../01_Crafting-Interpreters/part02_jlox/chapter07_evaluating-expressions/README.md) |
| 静态类型 | **Rust / rustc** |

---

## 自测

- [ ] 举 3 个 CFG 不能查、ch4 能查的规则
- [ ] 合成属性 vs 继承属性 · 各举一例
- [ ] 强/静 vs 弱/动 对编译器意味着什么
- [ ] YACC 里 `$1` 与 `$$` 各是什么
- [ ] SDT 建 AST 相对保留完整 parse tree 的好处

---

## 阅读进度

- [x] ch4 上下文相关分析（本章笔记）
- [ ] ch5 中间表示（Part II 开始）

---

## Part I 前端小结

| 章 | 产出 |
|:--:|------|
| 2 | Token 流 |
| 3 | 语法结构 / parse tree |
| 4 | AST + 类型 + 符号表 |
