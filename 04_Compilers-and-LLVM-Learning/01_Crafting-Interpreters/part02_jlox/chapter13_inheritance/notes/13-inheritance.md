# 第 13 章 · Inheritance（继承）

> 在线：[inheritance.html](https://craftinginterpreters.com/inheritance.html) · 中文：[第 13 章 继承](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch12 Classes](../chapter12_classes/notes/12-classes.md)

---

## 本章定位

**Part II 收官章**：补全 OOP 最后一块——**继承**与 **`super`**。至此 **jlox**（Java 树遍历解释器）功能完备；下一章起进入 **Part III · clox**（C 字节码 VM）。

| 对比 | ch12 | ch13 |
|------|------|------|
| 类关系 | 单类 | **子类 `<` 超类** |
| 方法查找 | 本类 + 实例字段 | **沿超类链** |
| 关键字 | `this` | **`super`** |

| 小节 | 主题 |
|------|------|
| **§13.1** | 超类 / 子类 · `<` 语法 |
| **§13.2** | 类内隐式 **`super`** 局部变量 |
| **§13.3** | **`super.method()`** · `Expr.Super` · Resolver 检查 |
| **§13.4** | **Conclusion** · jlox 完结 |

---

## §13.1 超类与子类（Superclasses and Subclasses）

```lox
class Doughnut {
  cook() { print "Fry"; }
}
class BostonCream < Doughnut {
  cook() {
    super.cook();
    print "Fill with cream";
  }
}
```

| 语法 | 说明 |
|------|------|
| **`class Sub < Super { ... }`** | `<` 表示继承（Lox 无 `extends`） |
| **`LoxClass`** | 增加 **`superclass`** 指针（可为 `null`） |
| 方法暴露 | 子类实例查找方法时：**本类 → 超类 → …** |

**执行 / 声明阶段**：

- 解析超类名 → 运行时取已定义的 **`LoxClass`**。
- 子类 **`LoxClass`** 保存 **`superclass` 引用**；实例方法查找沿链向上。

**与 ch12 字段优先**：实例 **fields** 仍优先于类方法；继承只扩展 **方法表查找链**。

---

## §13.2 超类局部变量（A Superclass Local Variable）

嵌套类 + 闭包场景下，**`super`** 必须像 **`this`** 一样有稳定的词法绑定。

| 机制 | 说明 |
|------|------|
| **Resolver 进入 class 方法** | 若类有超类，在当前作用域 **`declare("super")`** |
| 运行时 | 方法环境中 **`super`** 指向 **超类 `LoxClass`**（非实例） |
| 目的 | 闭包内 **`super`** 解析到正确层级，不被动态遮蔽搞乱 |

**对照 ch11**：静态绑定 + 局部名 **`super`**，与 **`this`** 同一套 Resolver 模式。

---

## §13.3 调用超类方法（Calling Superclass Methods）

```lox
super.cook();           // Expr.Super + method name
super.cook(arg);        // 带参数调用
```

| 组件 | 说明 |
|------|------|
| **AST** | **`Expr.Super`**：method 名 + 可选 arguments |
| **查找** | 从 **`super` 绑定的超类** 起找方法（跳过子类 override） |
| **`this` 绑定** | 调用超类方法时 **`this` 仍是当前实例** → 包装 **`LoxBoundMethod(this, superMethod)`** |

**Resolver 误用检查**：

| 非法 | 原因 |
|------|------|
| 类外 **`super`** | 无 **`super` 局部绑定 |
| 无超类的类内 **`super`** | 没有 **`superclass`** |
| （与 **`this`** 类似） | 解析期报错，非运行时才炸 |

**执行流程（示意）**：

```text
visitSuperExpr:
  superClass = environment.get("super")   // LoxClass
  method     = superClass.findMethod(name)
  bound      = LoxBoundMethod(thisInstance, method)
  bound.call(args...)
```

---

## §13.4 结论（Conclusion）

**jlox 管线（完整）**：

```text
Source → Scanner → Parser → AST
              → Resolver（语义分析）
              → Interpreter（Visitor 树遍历）
```

| 已实现（Part II） | 技术要点 |
|------------------|----------|
| 表达式 / 语句 / 控制流 | Visitor · Environment 链 |
| 函数 / 闭包 | `LoxFunction` · closure · distance |
| 类 / 实例 / `this` / `init` | `LoxBoundMethod` |
| **继承 / `super`** | 超类链 · `Expr.Super` |

**为何转向 clox？**

- AST 树遍历：**指针追逐多、缓存不友好、间接层厚**。
- Part III 用 **C + 字节码 + 栈式 VM**：更紧凑、更快、更贴近「真实编译器 / VM」实现。

**下一章**：[ch14 Chunks of Bytecode](../../../part03_clox/chapter14_chunks-of-bytecode/) — Part III 起点。

---

## Part II 能力总览（ch4～13）

```text
扫描 → AST → 求值 → 状态 → 控制流 → 函数/闭包
  → Resolver → 类/OOP → 继承/super
         jlox 完结 ✓
```

---

## 本章速记

```text
§13.1  class Sub < Super · LoxClass.superclass · 方法链查找
§13.2  Resolver 在方法内 declare super → 超类 LoxClass
§13.3  Expr.Super · this 不变 · 超类方法 · 误用检查
§13.4  jlox 完结 → Part III clox
```

---

## 读后下一步

| 部分 | 章 | 内容 |
|------|:--:|------|
| **III** | **14** | [Chunks of Bytecode](../../../part03_clox/chapter14_chunks-of-bytecode/) · **`Chunk`** · 常量池 |
| **III** | **15** | VM · **ip** · 值栈 |
| **III** | **16** | 按需扫描 · 关键字 DFA |

---

## 自测

1. 子类 override 方法后，`super.foo()` 解析到哪一层的方法？
2. 为什么 `super` 要像 `this` 一样在类方法作用域里单独绑定？
3. jlox 与 clox 在「表示程序」上的根本差异是什么？

---

## 阅读进度

- [x] §13.1～§13.4 结构梳理（本章笔记）
- [ ] 对照 jlox：`LoxClass.findMethod` 与 `visitSuperExpr`
- [ ] 本章 Challenges · 回顾 Part II 总练习
