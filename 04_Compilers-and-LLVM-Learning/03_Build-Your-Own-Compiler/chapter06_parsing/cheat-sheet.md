# 第 6 章 · 语法分析 · 速记与自测

← [本章目录](./README.md) · 上一节：[04-terms.md](./04-terms.md)

---

## 本章速记

```text
§1  compilation_unit = import* + top_defs · 后置 * [] · defun/defvars/struct/typedef
§2  stmts/stmt(13) · if + LOOKAHEAD(1) else
§3  expr→expr10→…→expr1 优先级塔
§4  unary → postfix* → primary
```

---

## 三句背诵

1. **文件：import + 顶层定义；C♭ 类型修饰符后置。**
2. **语句多分支；if 的 else 用 LOOKAHEAD(1)。**
3. **表达式分层；项分 unary/postfix/primary。**

---

## 非终端地图

| 层级 | 代表规则 |
|------|----------|
| 文件 | `compilation_unit` |
| 语句 | `stmts` · `stmt` |
| 表达式 | `expr` … `expr1` |
| 项 | `unary` · `postfix` · `primary` |

---

## 自测

- [ ] 四种 top_defs
- [ ] dangling else 在 ch6 哪条规则解决
- [ ] 优先级为何用多层 expr 而非单规则
- [ ] postfix 链如何解析 `f()[0]`

---

## 阅读进度

- [x] ch6 语法分析 — **第1部分 代码分析 完成**
- [ ] ch7 JavaCC 的 action 和抽象语法树（第2部分开始）
