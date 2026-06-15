# 第 27 章 · Classes and Instances（类与实例）

> 在线：[classes-and-instances.html](https://craftinginterpreters.com/classes-and-instances.html) · 中文：[第 27 章 类和实例](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch26 GC](../chapter26_garbage-collection/notes/26-garbage-collection.md)

> **原书对照**：实例字段与 **`OP_GET/SET_PROPERTY`** 在 **第 25 章 *Objects*** 亦有详述；本章笔记合并「类 + 实例 + 属性」学习批次。

---

## 本章定位

**GC（ch26）** 就绪后，安全引入 **OOP**：类是一等堆对象，**调用类 = 构造实例**；实例用 **哈希表** 存字段。

| 对比 | jlox ch12 | clox ch27 |
|------|-----------|-----------|
| 类 | **`LoxClass`** | **`ObjClass`** |
| 实例化 | **`Breakfast()`** | **`OP_CALL`** 判别 **`ObjClass`** |
| 字段 | **`LoxInstance.fields`** | **`ObjInstance.fields` Table** |
| 语法 | **`Expr.Get/Set`** | **`OP_GET/SET_PROPERTY`** |

| 主题 | 要点 |
|------|------|
| **Class Objects** | **`OP_CLASS`** · **`ObjClass`** |
| **Instances** | 无 **`new`** · call class |
| **Properties** | **`.`** · 实例 **Table** |

---

## 类对象（Class Objects）

```lox
class Pair {
  // 方法 ch28
}
```

**编译**：

```text
OP_CLASS  name_constant
→ runtime: ObjClass* 入常量 / 栈
→ OP_DEFINE_GLOBAL 等绑定类名
```

**`ObjClass`**（堆 `Obj` 子类型）：

| 字段 | 说明 |
|------|------|
| **`name`** | **`ObjString*`** |
| **`methods`** | **`Table`**（ch28 填方法） |

**类本身也是 Value**：可在运行时传递、赋值（与 jlox 一致）。

---

## 类的实例化（Instances）

**无 `new`**：

```lox
var pair = Pair();
```

**`OP_CALL`** 路径扩展：

```text
callee = ObjClass
  → 分配 ObjInstance
  → instance.class = 该类
  → push(instance) 作为「构造结果」
  （init / 方法 ch28）
```

| 要点 | 说明 |
|------|------|
| **与函数共用 `OP_CALL`** | arity 校验 · CallFrame（`init` 时 ch28） |
| **`ObjInstance`** | 新堆对象 · **`fields` 空 Table** |

**对照 jlox [ch12 §12.2](../../part02_jlox/chapter12_classes/notes/12-classes.md)**：类作 **callable 工厂**。

---

## 字段与属性（Get and Set Expressions）

```lox
pair.first = 1;
print pair.first;
```

**编译**：

| 语法 | 字节码 |
|------|--------|
| **`obj.field`** | compile `obj` → **`OP_GET_PROPERTY`** name |
| **`obj.field = val`** | compile `obj` → compile `val` → **`OP_SET_PROPERTY`** name |

**运行时**：

```text
OP_GET_PROPERTY:
  instance = pop 必须是 ObjInstance
  name = constant string
  tableGet(instance.fields, name) → push

OP_SET_PROPERTY:
  value, instance = ...
  tableSet(instance.fields, name, value)
  push value（赋值表达式值）
```

| 设计 | 说明 |
|------|------|
| **每实例一个 Table** | 动态增删字段 · 类似 JS 对象 |
| **键 intern 字符串** | ch20 指针相等 · GC 弱引用表协同 |
| **方法 vs 字段** | ch28：**字段优先** 于同名方法 |

**对照 jlox ch12 §12.3～§12.4**：Get/Set AST → clox **opcode + 实例表**。

---

## 对象模型（clox OOP 栈）

```text
ObjClass (methods Table)
    │
    │ OP_CALL / 工厂
    ▼
ObjInstance (fields Table)
    │
    │ OP_GET/SET_PROPERTY
    ▼
  动态字段读写
```

**GC**：**`markClass` / `markInstance`** 标记 **name、methods、fields** 中 Value 引用的 Obj。

---

## 本章速记

```text
OP_CLASS     创建 ObjClass · 绑定类名
实例化       OP_CALL + ObjClass → ObjInstance
属性         OP_GET/SET_PROPERTY · instance.fields 哈希表
无 new       类当 callable
GC           mark 遍历 class/methods/fields
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **28** | [chapter28 · Methods](../chapter28_methods/) | 方法 · **`this`** · **`OP_INVOKE`** |
| **13** jlox | Inheritance | **`super`** · clox 后续章 |
| **25** 原书 | Objects | 与实例字段 opcode 重叠阅读 |

---

## 自测

1. `Pair()` 在字节码层与 `foo()` 如何区分 callee 类型？
2. 实例字段为何用 Table 而非固定 struct 字段？
3. GC mark 为何要扫描 `instance.fields` 里的 Value？

---

## 阅读进度

- [x] Class / Instance / Property 结构梳理（本章笔记）
- [ ] 对照 jlox ch12 画 clox opcode 路径
- [ ] 本章 + 原书 ch25 Challenges
