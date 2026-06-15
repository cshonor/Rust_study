# 第 5 章 · Representing Code（表示代码 / AST） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
§5.1  正则不够 → CFG/BNF · | * + ?
§5.2  Expr 子类 · GenerateAst 生成代码
§5.3  Visitor + 双重分派 · 操作与节点分离
§5.4  AstPrinter → (+ 1 (* 2 3)) 验 parser
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **6** | [chapter06 · Parsing Expressions](../chapter06_parsing-expressions/) | **递归下降** · Token → AST |
| **7** | Evaluating | Visitor **解释执行** |

动手 ch6 前：运行 **`GenerateAst`** 生成 AST 类 · 扫一眼 **附录 II**。

---

---

## 自测 / 对照（可选）

- [ ] 用 BNF 写一条 Lox **二元表达式**规则（含 `*` 重复）。
- [ ] 解释：为何 `1 + (2 * 3)` 不能只用正则扫描器完成，还需要 parser？
- [ ] 画双重分派：`Binary.accept(interpreter)` 如何调到 `Interpreter.visitBinary`？
- [ ] 对 `print 1 + 2;` 的预期 AstPrinter 输出是什么？（需 ch6 后才有完整语句树）

---

---

## 阅读进度

- [x] §5.1～§5.4 结构梳理（本章笔记）
- [ ] 运行 `GenerateAst.java` / 对照附录 II
- [ ] 实现或阅读 `AstPrinter`
- [ ] 本章 Challenges
