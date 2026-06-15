# 第 13 章 · Inheritance（继承） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ch4-13.md](./05-ch4-13.md)

---

## 本章速记

```text
§13.1  class Sub < Super · LoxClass.superclass · 方法链查找
§13.2  Resolver 在方法内 declare super → 超类 LoxClass
§13.3  Expr.Super · this 不变 · 超类方法 · 误用检查
§13.4  jlox 完结 → Part III clox
```

---

---

## 读后下一步

| 部分 | 章 | 内容 |
|------|:--:|------|
| **III** | **14** | [Chunks of Bytecode](../../part03_clox/chapter14_chunks-of-bytecode/) · **`Chunk`** · 常量池 |
| **III** | **15** | VM · **ip** · 值栈 |
| **III** | **16** | 按需扫描 · 关键字 DFA |

---

---

## 自测

1. 子类 override 方法后，`super.foo()` 解析到哪一层的方法？
2. 为什么 `super` 要像 `this` 一样在类方法作用域里单独绑定？
3. jlox 与 clox 在「表示程序」上的根本差异是什么？

---

---

## 阅读进度

- [x] §13.1～§13.4 结构梳理（本章笔记）
- [ ] 对照 jlox：`LoxClass.findMethod` 与 `visitSuperExpr`
- [ ] 本章 Challenges · 回顾 Part II 总练习
