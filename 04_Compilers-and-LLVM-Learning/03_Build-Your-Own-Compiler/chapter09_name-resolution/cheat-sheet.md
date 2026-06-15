# 第 9 章 · 语义分析（1）引用的消解 · 速记与自测

← [本章目录](./README.md) · 上一节：[03-type-name-resolution.md](./03-type-name-resolution.md)

---

## 本章速记

```text
§1  语义五阶段(严序) · Visitor/ASTVisitor · Local/Type/TypeChecker
§2  LocalResolver · Scope栈 · 内→外查 · VariableNode→定义
§3  TypeResolver · TypeTable · TypeRef→Type · 类型无嵌套域
```

---

## 三句背诵

1. **语义分五步；本章做变量名与类型名消解。**
2. **Visitor 把 Pass 逻辑收到 Resolver/Checker 里。**
3. **变量用 Scope 向上查；类型用 TypeTable 查。**

---

## 对照表

| 组件 | 任务 |
|------|------|
| LocalResolver | 变量引用 → 定义 |
| TypeResolver | TypeRef → Type |
| ToplevelScope / LocalScope | 变量作用域 |
| TypeTable | 类型名映射 |

---

## 自测

- [ ] 五阶段顺序及后三步在哪章
- [ ] Visitor 模式解决什么问题
- [ ] 变量查找从内到外过程
- [ ] 类型为何不用 Scope 树

---

## 阅读进度

- [x] ch9 语义分析（1）引用的消解
- [ ] ch10 语义分析（2）静态类型检查
