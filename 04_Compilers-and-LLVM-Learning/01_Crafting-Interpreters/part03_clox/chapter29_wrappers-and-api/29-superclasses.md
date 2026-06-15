# 第 29 章 · Superclasses（超类与继承）

> 在线：[methods.html](https://craftinginterpreters.com/methods.html)（继承 / super 节） · 中文：[第 28～29 章相关](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch28 Methods](../chapter28_methods/28-methods.md)

> **原书对照**：clox **继承 / `super`** 实现位于 **Methods 章后半**（与 ch28 同页滚动阅读）。原书 **第 29 章** 题为 *Wrappers and API*（REPL / C API），见 [wrappers-and-api.html](https://craftinginterpreters.com/wrappers-and-api.html) · 本笔记按学习批次「超类」单独整理。

---

## 本章定位

OOP **最后一块**：**`<` 继承** · **copy-down 方法表** · **`super`** 静态绑定 · **`OP_SUPER_INVOKE`**。

| 对比 | jlox ch13 | clox ch29 |
|------|-----------|-----------|
| 语法 | **`class Sub < Super`** | 同 |
| 方法查找 | **超类链** 动态 | **子类表已含拷贝** · 一次 hash |
| **`super`** | **`Expr.Super`** · Resolver | **`OP_GET_SUPER`** · 编译期 **`super` 局部** |
| 优化 | — | **`OP_SUPER_INVOKE`** |

| 小节 | 主题 |
|------|------|
| **§29.1** | **`<` 继承** · **`OP_INHERIT`** · copy-down |
| **§29.2** | 隐式 **`super`** 局部 |
| **§29.3～§29.4** | **`OP_GET_SUPER`** · **`OP_SUPER_INVOKE`** |

---

## §29.1 继承（Inheritance）

```lox
class Doughnut { cook() { print "Fry"; } }
class BostonCream < Doughnut {
  cook() { super.cook(); print "Cream"; }
}
```

**编译**：**`class BostonCream < Doughnut`** → 超类名常量 + **`OP_SUBCLASS`** / **`OP_INHERIT`** 序列（以书为准）。

**Copy-down inheritance（向下拷贝）**：

```text
OP_INHERIT:
  将 superclass.methods 表中所有条目
  复制到 subclass.methods（子类未 override 的键）
```

| 优点 | 说明 |
|------|------|
| **运行时查找 O(1) 一次 hash** | 无需沿继承链 walk |
| **override** | 子类声明同名方法 → **覆盖** 拷贝项 |

| 代价 | 说明 |
|------|------|
| **声明时拷贝** | 超类后改方法 **不影响** 已声明子类（Lox 语义可接受） |
| **内存** | 方法闭包 **共享引用** · 非 deep copy chunk |

**对照 jlox [ch13](../../part02_jlox/chapter13_inheritance/13-inheritance.md)**：运行时 **超类链查找** vs clox **表拷贝** —— 典型 **编译期换运行时** 优化。

---

## §29.2 超类局部变量（A Superclass Local Variable）

**问题**：子类方法内 **`super.cook()`** · 嵌套闭包也要解析 **`super`** → 不能仅靠栈帧临时查表。

**编译器**（类比 jlox ch13 §13.2）：

- 编译 **带超类的 class 方法** 时，在外层作用域 **`addLocal("super")`**。
- **`super`** 绑定为 **`ObjClass*`**（**超类对象** · 非实例）。

| 效果 | 说明 |
|------|------|
| **闭包 upvalue** | 可捕获 **`super` 类引用** |
| **静态 slot / upvalue 索引** | 与 **`this` slot 0** 并列的另一绑定 |

---

## §29.3～§29.4 Super 表达式与优化

**`super.method`** · **`super.method(args)`**

| 朴素 | 优化 |
|------|------|
| **`OP_GET_SUPER` name** | **`OP_SUPER_INVOKE` name argCount** |

**`OP_GET_SUPER`**：

```text
从当前方法的 super 绑定（local/upvalue）取 ObjClass*
在该类 methods 表查 name（跳过子类 override）
→ 得到 ObjClosure* · 配合 this(slot0) 调用
```

| 关键 | 说明 |
|------|------|
| **不从 instance 类查** | 从 **super 指向的超类** 查 → **跳过子类重写** |
| **`this` 不变** | 仍是 **当前实例** slot 0 |

**`OP_SUPER_INVOKE`**：

- 同 **`OP_INVOKE`** 融合思想：**super 查找 + call** 一条指令。
- 避免 **BoundMethod** 等中间对象。

**对照 ch28 §28.5**：**Invoke 族超指令** 扩展到 **super 路径**。

---

## 继承 + super 数据流

```text
声明时:
  OP_INHERIT  →  superclass.methods 拷贝进 subclass.methods

调用 super.cook():
  OP_SUPER_INVOKE "cook" 0
    super 类表 → cook 闭包
    slot0 = this 实例
    执行超类方法体
```

---

## 本章速记

```text
§29.1  < Super · OP_INHERIT copy-down · 一次 hash 查方法
§29.2  super 局部/upvalue · 超类 ObjClass*
§29.3  OP_GET_SUPER · 从超类表查 · 非子类
§29.4  OP_SUPER_INVOKE · 融合 super+调用
原书29  Wrappers and API · REPL/C 嵌入 · 另读官网
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **30** | [chapter30 · Optimization](../chapter30_optimization/) | **`& (cap-1)`** · **NaN boxing** |
| **29 原书** | [Wrappers and API](https://craftinginterpreters.com/wrappers-and-api.html) | **`run()` REPL** · embed API |
| **13** jlox | Inheritance | 超类链 vs copy-down 对照 |

---

## 自测

1. copy-down 后，子类还能否调用被 override 的超类方法？（通过什么？）
2. `OP_GET_SUPER` 为何不能等价于 `OP_GET_PROPERTY`？
3. `OP_SUPER_INVOKE` 相对 super Get+Call 省什么？

---

## 阅读进度

- [x] §29.1～§29.4 结构梳理（本章笔记）
- [ ] 对照 jlox ch13 画 copy-down vs 链式查找
- [ ] 阅读原书 Wrappers and API（可选）
