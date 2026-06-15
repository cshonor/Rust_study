# 第 8 章 · 抽象语法树的生成 · 速记与自测

← [本章目录](./README.md) · 上一节：[04-parser-startup.md](./04-parser-startup.md)

---

## 本章速记

```text
§1  literal/Variable · TypeRef/Type · Binary*左 · Assign递归右
§2  If/While/Block(locals+stmts)
§3  DefinedVariable列表 · DefinedFunction · AST根 · import→.hb
§4  Parser构造 · parseFile/parse · compilation_unit()
```

---

## 三句背诵

1. **自下而上 action：primary 先，AST 根最后。**
2. **左结合 `*`，右结合 `=` 用递归 AssignNode。**
3. **parse() 调 compilation_unit() 得 AST。**

---

## 节点地图

| 层级 | 代表节点 |
|------|----------|
| 表达式 | Literal · Variable · BinaryOp · Assign |
| 语句 | If · While · Block |
| 声明 | DefinedVariable · DefinedFunction |
| 根 | AST |

---

## 自测

- [ ] TypeRef 与 Type 分开的原因
- [ ] `i=j=1` 树形
- [ ] BlockNode 两字段
- [ ] import 加载什么文件

---

## 阅读进度

- [x] ch8 抽象语法树的生成
- [ ] ch9 语义分析（1）引用的消解
