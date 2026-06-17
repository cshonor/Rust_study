# 第 4 章 · Scanning（扫描 / 词法分析） · 速记与自测

← [本章目录](./README.md) · 上一节：[08-scanning.md](./08-scanning.md)

---

## 本章速记

```text
§4.1  main: 0→REPL · 1→runFile · >1→exit(64) · 有错 exit(65)
       runFile / REPL · hadError · error(line) → report · 有错不执行残缺代码
§4.2  Lexeme = 纯文本片段 · Token = type + lexeme + literal + line · Parser 只吃 Token
§4.4  start / current / line
§4.5  switch 单字符 · 词法错误继续扫 · match 双字符
§4.6  // 注释 · 字符串 · double · peekNext
§4.7  maximal munch · Map 查保留字
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **5** | [chapter05 · Representing Code](../chapter05_representing-code/) | 定义 **AST 节点**（树的数据结构） |
| **6** | Parsing | Token → AST |

写 parser 前务必完成 ch5 的表达式/语句节点类（附录 II 有生成代码参考）。

---

---

## 自测 / Challenges（可选）

- [ ] 说明 jlox 退出码 **64** / **65** 与 REPL 下 `hadError` 重置策略。
- [ ] 画 `Scanner` 在 `"var x = 1.5; // hi\n"` 上的 `start/current/line` 变化。
- [ ] 列举 3 个需 **lookahead** 的 Lox 词素。
- [ ] 说明 **maximal munch** 若不用，关键字 `or` 与标识符 `orchid` 会如何冲突。
- [ ] 对比 **03**《自制编译器》：JavaCC 生成扫描器 vs 本书手写——各牺牲/得到什么。

---

---

## 阅读进度

- [x] §4.1～§4.7 结构梳理（本章笔记）
- [ ] 跟书实现 / 对照 GitHub `jlox` 源码
- [ ] 本章 Challenges
