# 第 12 章 · Classes（类）

> 在线：[classes.html](https://craftinginterpreters.com/classes.html) · 中文：[第 12 章 类](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part II · [part02_jlox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch11 Resolving](../chapter11_resolving-and-binding/notes/11-resolving-and-binding.md)

---

## 本章定位

为 Lox 加入 **基于类的 OOP**：类声明、实例化、字段、方法、**`this`**、**`init`**。除 **继承（ch13）** 外，OOP 核心在此章齐备。

| 运行时对象 | 角色 |
|------------|------|
| **`LoxClass`** | 类对象（方法表 + 可调用工厂） |
| **`LoxInstance`** | 实例（字段 HashMap） |
| **`LoxBoundMethod`** | 已绑定 `this` 的方法 |

| 小节 | 主题 |
|------|------|
| **§12.1** | **`class`** 声明 · **`LoxClass`** |
| **§12.2** | 实例化 · 类即工厂（无 `new`） |
| **§12.3** | **`.`** Get / Set · 实例字段 |
| **§12.4** | 类方法 · **`LoxBoundMethod`** · 字段优先 |
| **§12.5** | **`this`** · 方法环境 · Resolver 检查 |
| **§12.6** | **`init`** · 构造 / 初始化器规则 |

---

## §12.1 类声明（Class Declarations）

```lox
class Breakfast {
  cook() {
    print "Eggs!";
  }
}
```

- **`class Name { methods... }`** → **`Stmt.Class`**（name + methods 列表）。
- 执行：`define(name, new LoxClass(...))`，类名与 **`fun`** 一样进入环境。
- **`LoxClass`**：保存类名 + **方法名 → LoxFunction** 的映射。

---

## §12.2 创建实例（Creating Instances）

Lox **没有 `new` 关键字**：

```lox
var breakfast = Breakfast();  // 把类当 callable
```

| 步骤 | 说明 |
|------|------|
| **`LoxClass` 实现 `LoxCallable`** | `call()` 创建 **`LoxInstance`** |
|  arity | 通常 **0**（构造逻辑放 `init`） |
| 返回 | 新实例对象 |

**对照**：JavaScript 里 `new Foo()` vs Lox 直接 **`Foo()`** —— 语法更简，语义仍是通过类生成实例。

---

## §12.3 实例上的属性（Properties on Instances）

**`.` 语法**：

```lox
instance.field      // Get
instance.field = v  // Set
```

| AST | 执行 |
|-----|------|
| **`Expr.Get`** | object 求值 → 在 **`LoxInstance`** 上查字段 |
| **`Expr.Set`** | 先求 object 与 value → **set** 字段 |

**`LoxInstance`**：内部 **`Map<String, Object> fields`**，可 **动态增删** 字段（类似 JS 对象扩展）。

**查找顺序（Get）**（§12.4 细化）：

1. 实例 **fields**
2. 否则类 **methods**

---

## §12.4 类上的方法（Methods on Classes）

- 方法在类声明体中定义为 **`fun`-风格**，存于 **`LoxClass`**。
- **Get `obj.method`**：
  - 若 **fields** 有同名键 → **字段优先**（覆盖方法名）。
  - 否则取类方法 → 包装为 **`LoxBoundMethod(instance, method)`**。

**`LoxBoundMethod`**：

| 实现 | 说明 |
|------|------|
| **`LoxCallable`** | 调用时执行底层 **`LoxFunction`** |
| 绑定 | 携带 **`this` 实例**（§12.5） |

用户代码：`bacon.cook()` → Get 得到 bound method → Call。

---

## §12.5 This

**`this`** 关键字：方法内指向 **当前实例**。

| 机制 | 说明 |
|------|------|
| 调用 bound method | 创建 **新 Environment**，**`define("this", instance)`** |
| 方法体 | 在该环境中执行，`this` 与参数、局部变量一样 lookup |
| **Resolver** | 进入 **class method** 时 **`resolveLocal(..., "this")`**；类外使用 **`this`** → **resolution error** |

**与闭包关系**：方法体仍是 **`LoxFunction`**，可捕获外层变量；**`this` 环境** 在 **每次调用** 时叠加在 closure 之上（或作为 innermost 层）。

```text
call boundMethod:
  env(enclosing = method.closure)
    this = instance
    params...
    execute body
```

---

## §12.6 构造函数与初始化器（Constructors and Initializers）

约定：实例化后若类定义了 **`init`**，**自动调用**。

```lox
class Foo {
  init(a) {
    this.a = a;
  }
}
var f = Foo(1);
```

| 规则 | 说明 |
|------|------|
| 自动调用 | **`LoxClass.call()`** 创建实例后查找 **`init`** 并 invoke |
| **`return;` in init** | 隐式 **返回 `this`**（实例） |
| **禁止** | **`init` 中 `return expr;`**（expr 非空）→ Resolver / 运行时限制 |
|  arity | **`init`** 参数个数决定 **`Foo(...)` 所需参数** |

防止用户把 **`init`** 写成普通函数返回任意值，破坏「构造总是得到实例」的不变式。

---

## OOP 能力矩阵（ch12 vs ch13）

| 特性 | ch12 | ch13 |
|------|:----:|:----:|
| class / instance | ✓ | ✓ |
| fields / methods / this | ✓ | ✓ |
| init | ✓ | ✓ |
| **inheritance / super** | — | ✓ |

---

## 本章速记

```text
§12.1  Stmt.Class · LoxClass
§12.2  Breakfast() 无 new · LoxInstance
§12.3  Get/Set · instance.fields Map
§12.4  字段优先 · LoxBoundMethod
§12.5  this 环境 · Resolver 限制
§12.6  init 自动调用 · 禁 return 值
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **13** | [chapter13 · Inheritance](../chapter13_inheritance/) | **`super`** · 方法继承 |
| **14+** | [part03_clox](../../part03_clox/) | 字节码 VM 重新开始 |

---

## 自测

1. 实例字段与方法同名时，`obj.name` 解析到哪？
2. 为什么 `init` 要单独限制 `return`？
3. `LoxBoundMethod` 和裸 `LoxFunction` 在调用路径上差在哪一步？

---

## 阅读进度

- [x] §12.1～§12.6 结构梳理（本章笔记）
- [ ] 画 `bacon.cook()` 的 Get → Call → this 环境链
- [ ] 本章 Challenges
