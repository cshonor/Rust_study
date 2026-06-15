# 第 28 章 · Methods and Initializers（方法与初始化器）

> 在线：[methods.html](https://craftinginterpreters.com/methods.html) · 中文：[第 28 章 方法](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch27 Classes](../chapter27_classes-and-instances/notes/27-classes-and-instances.md)

---

## 本章定位

为 **OOP 注入可调用行为**：类方法表 · **`this` 作 slot 0** · **`init`** · **`OP_INVOKE` 超指令** 避免 BoundMethod 堆分配。

| 对比 | jlox ch12 | clox ch28 |
|------|-----------|-----------|
| 方法查找 | 实例 fields 优先 → 类方法 | 同 · **`ObjClass.methods` Table** |
| 绑定 | **`LoxBoundMethod`** 常分配 | 一般路径 BoundMethod · **invoke 优化** |
| **`this`** | 方法环境 **`define("this")`** | **隐藏局部变量 slot 0** |
| **`init`** | 自动调用 · return 限制 | 同语义 · 编译期约束 |

| 小节 | 主题 |
|------|------|
| **§28.1** | 方法声明 · **`methods` 哈希表** |
| **§28.2** | **`ObjBoundMethod`** |
| **§28.3** | **`this` = slot 0** |
| **§28.4** | **`init()`** 初始化器 |
| **§28.5** | **`OP_INVOKE`** 超指令 |

---

## §28.1 方法声明（Method Declarations）

```lox
class Brunch {
  init(food) { this.food = food; }
  eat() { print this.food; }
}
```

**编译 `class` 体中的方法**：

- 每个方法 → 编译为 **`ObjFunction` + `ObjClosure`**（含 upvalue）。
- **`tableSet(&klass->methods, name, closure)`**。

| 结构 | 说明 |
|------|------|
| **`ObjClass.methods`** | **`Table`**：键 **`ObjString*`** 方法名 · 值 **`ObjClosure*`** |
| 实例字段 | 仍在 **`ObjInstance.fields`**（ch27）· **优先于同名方法** |

---

## §28.2 绑定方法（Bound Methods）

**朴素路径**：`instance.method` **仅 Get** → 查 **`methods` 表** → 包装 **`ObjBoundMethod { receiver, method }`** → push。

| 对象 | 作用 |
|------|------|
| **`ObjBoundMethod`** | **胶囊**：**receiver 实例** + **方法闭包** |
| **`OP_CALL`** | 对 BoundMethod：绑定 **`this`**（slot 0）再进 chunk |

**代价**：每次 **Get 属性** 若只为调用 → **堆分配 BoundMethod** · GC 压力 · 慢。

**§28.5** 用 **`OP_INVOKE`** 规避（见下）。

**对照 jlox [ch12 §12.4](../../part02_jlox/chapter12_classes/notes/12-classes.md)**：语义相同 · clox 后加 **invoke 融合优化**。

---

## §28.3 This 关键字（This）

**技巧**：**不单独 opcode** —— **`this` 是编译器合成的局部变量，固定占 slot 0**。

| 约定 | 说明 |
|------|------|
| **CallFrame slot 0** | 方法调用时 **receiver（实例）** 放在 **第一个局部槽** |
| 方法内 **`this`** | 编译为 **`OP_GET_LOCAL 0` / `OP_SET_LOCAL 0`** |
| 形参 | 从 **slot 1** 起 |

```text
OP_CALL method on instance:
  stack: [ ..., instance, arg1, arg2 ]
  frame.slots[0] = instance  → this
  frame.slots[1..] = params
```

| 优点 | 说明 |
|------|------|
| **复用局部变量机制** | 无 **`this` 专用指令** |
| **O(1)** | 与 **`GET_LOCAL`** 同速 |

**编译器**：进入方法体时 **`addLocal("this")`** 并 **`markInitialized`**（或等价固定 slot 0）。

**对照 jlox**：Resolver 限制 **`this`** 仅在方法内；clox 靠 **slot 0 约定 + 编译器**。

---

## §28.4 初始化器（Initializers）

**`init`**：实例化时 **自动 invoke**（在 **`ObjClass` + `OP_CALL`** 路径上检测 **`init` 方法**）。

| 规则 | 说明 |
|------|------|
| **自动调用** | `Klass()` 后若存在 **`"init"`** → **`OP_INVOKE` / call** |
| **隐式 `return this`** | **`init` 末尾** 编译 **`OP_GET_LOCAL 0` + `OP_RETURN`** |
| **禁止 `return expr`** | 编译期错误（expr 非空） |
| **arity** | **`init` 参数个数** = **`Klass(...)` 所需实参** |

**对照 jlox ch12 §12.6**：语义一致。

---

## §28.5 优化的调用（Optimized Invocations · Superinstruction）

**模式**：源码 **`obj.method(args)`** —— 若拆成 **GET_PROPERTY + CALL** → 中间 **BoundMethod** 分配。

**窥孔优化（Peephole）**：Parser/Compiler 见到 **`.` 后紧跟 `(`** → 直接 **`OP_INVOKE`**。

| 指令 | 行为 |
|------|------|
| **`OP_INVOKE name argCount`** | **一次完成**：在 **instance** 上按名查 **方法** · 设置 **slot 0 = this** · **call** · **无 BoundMethod 分配** |

```text
// 慢路径
GET_PROPERTY "eat"
CALL 0          // BoundMethod 已堆分配

// 快路径
OP_INVOKE "eat" 0
```

**书中基准**：方法密集代码 **~7×+** 提升（相对 naive Get+Call）。

**概念**：**Superinstruction** = 融合多条语义等价 opcode 为一条 **更高层指令**（仍是一次 dispatch）。

---

## 方法调用路径（小结）

```text
obj.m(a):
  优化: OP_INVOKE "m" 1
  朴素: GET_PROPERTY → ObjBoundMethod → OP_CALL

方法体:
  slot0 = this
  OP_GET/SET_LOCAL / fields / upvalues ...
  OP_RETURN
```

---

## 本章速记

```text
§28.1  class.methods Table · ObjClosure
§28.2  ObjBoundMethod · receiver+method
§28.3  this = slot 0 · GET/SET_LOCAL 0
§28.4  init 自动调用 · 只 return this
§28.5  OP_INVOKE 融合查+调 · 免堆分配
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **29** | [chapter29 · Superclasses](../chapter29_wrappers-and-api/notes/29-superclasses.md) | **`OP_INHERIT`** · **`super`** |
| **30** | [chapter30 · Optimization](../chapter30_optimization/) | 掩码 · **NaN boxing** |

---

## 自测

1. 为何 `this` 用 slot 0 而不是单独 `OP_THIS`？
2. `OP_INVOKE` 相对 Get+Call 省掉了哪次堆分配？
3. 实例字段与方法同名时，`OP_INVOKE` 应命中哪一个？

---

## 阅读进度

- [x] §28.1～§28.5 结构梳理（本章笔记）
- [ ] 对照 benchmark 理解 invoke 提升
- [ ] 本章 Challenges
