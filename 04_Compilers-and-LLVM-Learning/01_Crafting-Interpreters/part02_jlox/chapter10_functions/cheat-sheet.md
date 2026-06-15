# 第 10 章 · Functions（函数） · 速记与自测

← [本章目录](./README.md) · 上一节：[07-ch8-10.md](./07-ch8-10.md)

---

## 本章速记

```text
§10.1  Call 后缀 · LoxCallable · arity · ≤255 args
§10.2  Native · clock()
§10.3  Stmt.Function · fun 解析
§10.4  LoxFunction · 每次 call 新 Environment
§10.5  Return 异常跳出 Visitor 栈
§10.6  closure 捕获定义时环境
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **11** | [chapter11 · Resolving](../chapter11_resolving-and-binding/) | **Resolver** · 静态作用域 · distance |
| **12** | [chapter12 · Classes](../chapter12_classes/) | `class` · `this` · OOP |

---

---

## 自测

1. 为什么 `()` 解析成「后缀运算符」而不是独立语句？
2. `return` 为什么用异常而不是改 Visitor 返回值类型？
3. 闭包里的 `enclosing` 指向哪里——定义处还是调用处？

---

---

## 阅读进度

- [x] §10.1～§10.6 结构梳理（本章笔记）
- [ ] 实现 / 对照 jlox 源码：`LoxFunction.call`、`Return`
- [ ] 本章 Challenges
