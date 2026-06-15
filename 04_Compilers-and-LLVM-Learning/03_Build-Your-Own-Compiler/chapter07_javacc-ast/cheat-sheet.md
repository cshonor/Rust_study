# 第 7 章 · JavaCC 的 action 和抽象语法树 · 速记与自测

← [本章目录](./README.md) · 上一节：[02-ast-and-nodes.md](./02-ast-and-nodes.md)

---

## 本章速记

```text
§1  action {} · Token(image/位置) · return 节点 · 与 | * + 组合
§2  Node→AST/Stmt/Expr · location/dump · BinaryOpNode · 不用 JJTree
```

---

## 三句背诵

1. **文法匹配检查；action 建 AST。**
2. **终端 = Token；非终端 = return 的 Node。**
3. **cbc 手写节点类，不要 JJTree 的 Node[]。**

---

## 对照表

| 概念 | 一句话 |
|------|--------|
| action | 匹配时执行的 `{ Java }` |
| 语义值 | 符号携带的对象/Token |
| `--dump-ast` | 打印 AST 调试 |
| JJTree | 自动生成但弱类型 |

---

## 自测

- [ ] 如何从 `<IDENTIFIER>` 取名字字符串
- [ ] 重复参数列表 action 为何执行多次
- [ ] StmtNode 与 ExprNode 分工
- [ ] 强类型子字段 vs 子节点数组

---

## 阅读进度

- [x] ch7 JavaCC 的 action 和抽象语法树
- [ ] ch8 抽象语法树的生成
