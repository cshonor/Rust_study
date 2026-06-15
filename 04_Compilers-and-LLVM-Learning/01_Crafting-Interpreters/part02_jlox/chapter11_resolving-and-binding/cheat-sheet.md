# 第 11 章 · Resolving and Binding（解析与绑定） · 速记与自测

← [本章目录](./README.md) · 上一节：[06-resolver-interpreter.md](./06-resolver-interpreter.md)

---

## 本章速记

```text
§11.1  词法作用域 · 动态查找闭包 Bug
§11.2  AST 后、运行前的语义分析趟
§11.3  Resolver + 作用域栈 declare/define
§11.4  getAt/assignAt(distance)
§11.5  顶层 return 等 resolution error
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **12** | [chapter12 · Classes](../chapter12_classes/) | **`class`** · 实例 · **`this`** |
| **13** | Inheritance | **`super`** · 继承 |

**clox 预告**：Part III 同样在编译期做 **Upvalue / 局部 slot** 绑定，思想与 Resolver 同源。

---

---

## 自测

1. distance = 0 表示变量在哪一层环境？
2. 为什么 `Resolver` 访问 `Expr.Literal` 可以是空操作？
3. 全局变量为何可以不进 `locals` map？

---

---

## 阅读进度

- [x] §11.1～§11.5 结构梳理（本章笔记）
- [ ] 手工 trace 一段嵌套 `var` + 闭包的 resolve 过程
- [ ] 本章 Challenges
