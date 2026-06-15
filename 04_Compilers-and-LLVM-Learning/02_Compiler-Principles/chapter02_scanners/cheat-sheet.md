# 第 2 章 · 扫描 · 速记与自测

← [本章目录](./README.md) · 上一节：[06-keywords-and-tradeoffs.md](./06-keywords-and-tradeoffs.md)

---

## 本章速记

```text
§1  字符流→Token流 · 过滤空白/注释 · 前端第一站
§2  RE：选择|连接|克林闭包* · 描述微语法
§3  NFA（多路径/ε）vs DFA（唯一转移）· 扫描器用 DFA
§4  Thompson → 子集构造 → Hopcroft 最小化
§5  表驱动（查表）vs 直接编码（switch/goto，常更快）
§6  关键字：先当标识符 · 散列表改词性 · 简化 DFA
```

---

## 三句背诵

1. **RE 说找什么，DFA 说怎么找；扫描器跑最小化 DFA。**
2. **Thompson → 子集构造 → Hopcroft，是 lex 类工具的理论链。**
3. **关键字别硬塞进 DFA，标识符 + 哈希表更工程。**

---

## 与 CI 对照

| 橡书 ch2 | CI |
|----------|-----|
| RE→DFA 理论 | jlox/clox **手写** Scanner |
| Token 概念 | [ch4 Scanning](../../../01_Crafting-Interpreters/part02_jlox/chapter04_scanning/README.md) · [ch16](../../../01_Crafting-Interpreters/part03_clox/chapter16_scanning-on-demand/README.md) |

---

## 自测

- [ ] 扫描器输入/输出各是什么？
- [ ] RE 三基本操作是什么？
- [ ] NFA 与 DFA 各一条区别
- [ ] 子集构造、Hopcroft 各解决什么问题
- [ ] 表驱动 vs 直接编码如何权衡
- [ ] 为何关键字用散列表而不是扩 DFA

---

## 阅读进度

- [x] ch2 扫描（本章笔记）
- [ ] ch3 语法分析
