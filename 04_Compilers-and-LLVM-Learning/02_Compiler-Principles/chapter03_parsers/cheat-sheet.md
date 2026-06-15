# 第 3 章 · 语法分析 · 速记与自测

← [本章目录](./README.md) · 上一节：[05-engineering-practice.md](./05-engineering-practice.md)

---

## 本章速记

```text
§1  CFG/BNF · 派生/分析树 · 歧义(悬挂else/优先级)→重写文法
§2  自顶向下 · 消左递归/提左因子 · 递归下降=LL(1) · CI jlox
§3  自底向上 · shift-reduce · 句柄 · LR(1) · yacc/bison
§4  项目集闭包+Goto → DFA → Action/Goto 表
§5  错误恢复 · LALR/SLR 表压缩 · 一元符/列表/上下文补 ch4
```

---

## 三句背诵

1. **ch2 单词，ch3 句子：CFG 描述，分析树/AST 记录结构。**
2. **手写走 LL 递归下降；工具走 LR 移入-归约 + Action/Goto 表。**
3. **歧义改文法；LR(1) 太大就 LALR 合并状态。**

---

## 与 CI 对照

| 橡书 ch3 | CI |
|----------|-----|
| CFG / 歧义 | [jlox ch5 CFG](../../../01_Crafting-Interpreters/part02_jlox/chapter05_representing-code/01-context-free-grammars.md) |
| 递归下降 | [jlox ch6](../../../01_Crafting-Interpreters/part02_jlox/chapter06_parsing-expressions/README.md) |
| 运算符解析 | [clox Pratt ch17](../../../01_Crafting-Interpreters/part03_clox/chapter17_compiling-expressions/02-a-pratt-parser.md) |

---

## 自测

- [ ] CFG 四要素 · BNF 是什么
- [ ] 歧义举例 + 一种消除法
- [ ] 左递归为何害自顶向下 · 两种文法改造
- [ ] shift vs reduce 各一句话
- [ ] Action 表 vs Goto 表
- [ ] LALR 相对 LR(1) 折中了什么

---

## 阅读进度

- [x] ch3 语法分析（本章笔记）
- [ ] ch4 上下文相关分析
