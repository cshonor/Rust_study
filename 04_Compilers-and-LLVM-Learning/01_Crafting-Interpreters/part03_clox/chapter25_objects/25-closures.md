# 第 25 章 · Closures（闭包）

> 在线：[calling-and-closures.html](https://craftinginterpreters.com/calling-and-closures.html)（Upvalue 节） · 中文：[第 24 章 调用和闭包](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 调用基础：[ch24 Calls](../chapter24_calling-and-closures/24-calling-and-closures.md) · 上一批：[ch24 §24.1～24.7](../chapter24_calling-and-closures/24-calling-and-closures.md)

> **原书对照**：Upvalue / 闭包在正文 **第 24 章后半**；本笔记按学习批次单独整理。原书第 25 章题为 *Objects*（实例字段），见 [ch27 Classes](../chapter27_classes-and-instances/27-classes-and-instances.md)。

---

## 本章定位

jlox 闭包靠 **Java GC + Environment 捕获**；clox 局部变量在 **栈槽** 上，函数返回即 **弹栈销毁** → 闭包若仍指向栈地址 = **悬空指针**。

| 对比 | jlox ch10～11 | clox 闭包 |
|------|---------------|-----------|
| 捕获 | `LoxFunction.closure` Environment | **`ObjUpvalue*`** 间接层 |
| 生命周期 | GC 管 Environment | **`OP_CLOSE_UPVALUE`** 迁堆 |
| 编译 | Resolver **distance** | **`resolveUpvalue`** · **`OP_GET/SET_UPVALUE`** |

| 主题 | 要点 |
|------|------|
| **Upvalue** | 闭包捕获变量的 **间接引用** |
| **Flattening** | 嵌套函数 **逐层传递** upvalue |
| **Closing** | 作用域结束 · 值 **栈 → 堆** |

---

## Upvalue（上值）

**问题**：内层函数引用外层 **局部变量**（在 caller 的 stack slot 上）；外层 **`OP_RETURN` 弹帧** 后 slot 失效。

**Lua 式解法**：**Upvalue** = 指向「变量所在位置」的 **间接层**，位置可以是 **栈槽** 或 **堆上 Upvalue 对象**。

| 类型 | 说明 |
|------|------|
| **`ObjUpvalue`** | 堆对象；**`location`** 指向 `Value*`（栈或自身 closed 字段） |
| **`ObjClosure`** | 包装 **`ObjFunction*`** + **`Upvalue* upvalues[]`** |
| 编译 | 被捕获的 local 标 **`isCaptured`**；emit **`OP_CLOSURE`** + upvalue 描述 |

**运行时读取**：

| 指令 | 行为 |
|------|------|
| **`OP_GET_UPVALUE i`** | 经 closure.upvalues[i]->location 读 Value |
| **`OP_SET_UPVALUE i`** | 写入同一 location |

**对照 jlox [ch11](../../part02_jlox/chapter11_resolving-and-binding/11-resolving-and-binding.md)**：`getAt(distance)` 静态绑定；clox 用 **upvalue 索引** 在字节码里固定。

---

## 扁平化 Upvalues（Flattening Upvalues）

**嵌套**：

```lox
fun outer() {
  var x = 1;
  fun middle() {
    fun inner() { print x; }
    return inner;
  }
  return middle();
}
```

**规则**：`inner` 要读 `outer.x` → **不只** outer 捕获；**middle 也须捕获 upvalue 并向下传**，形成 **upvalue 链 / 扁平化索引**。

| 编译 | 说明 |
|------|------|
| **`resolveUpvalue(compiler, name)`** | 先在 **本函数 locals** 找；否则 **递归外层 Compiler** |
| 每层函数 | 维护自己的 **upvalue 列表**；内层引用外层 = **外层 upvalue 槽位传递** |

**效果**：任意嵌套深度，闭包总能 **O(1) 索引** 到正确捕获，无需运行时按名链式查找。

---

## 闭合 Upvalue（Closing Upvalues）

**时机**：块/函数结束，**栈上被捕获的局部** 即将随 **`endScope` / `OP_RETURN`** 消失。

**编译**：在作用域结束前 emit **`OP_CLOSE_UPVALUE`**（对仍 open 的 upvalue）。

**运行时 `OP_CLOSE_UPVALUE`**：

```text
将 upvalue.location 指向的 Value 复制到 ObjUpvalue 内部（closed 存储）
location 改指向 closed 字段
该 upvalue 不再依赖栈
```

| 阶段 | upvalue.location 指向 |
|------|------------------------|
| **Open** | 栈上某 **`Value*`**（随 call frame 活） |
| **Closed** | **`ObjUpvalue` 内嵌 Value**（堆上，与帧无关） |

**外层已 return、栈已清空** 后，闭包仍通过 **closed upvalue** 安全读写的关键。

**VM 维护**： **`openUpvalues`** 链表（按栈位置排序），便于 scope 结束时 **批量 close**。

---

## ObjClosure 与 OP_CLOSURE

```text
compile fun:
  嵌套 Compiler → function chunk
  记录 upvalue 元数据（local slot / parent upvalue index）
  OP_CLOSURE constant(function) + upvalue count

run OP_CLOSURE:
  创建 ObjClosure + 填充 upvalues（open → 指向当前 frame slots）
  push closure Value
```

**调用闭包**：`OP_CALL` 目标为 **`ObjClosure`**（非裸 Function）→ 仍用 **CallFrame + chunk**，upvalues 挂在 closure 上。

---

## 闭包生命周期（总览）

```text
声明 outer 的 local x（栈槽）
  inner 捕获 → ObjUpvalue open → location = &stack[x]

outer 仍在执行：
  inner 读 x → 经 upvalue 读栈 ✓

outer return 前/块结束：
  OP_CLOSE_UPVALUE → x 值迁入堆上 ObjUpvalue

outer 已返回，栈回收：
  inner 仍持有 closed upvalue → 安全 ✓
```

---

## 本章速记

```text
Upvalue     间接层 · ObjClosure · GET/SET_UPVALUE
Flatten     嵌套时中间层传递 upvalue 索引
Close       OP_CLOSE_UPVALUE · open→closed · 防悬空
对照 jlox   Environment+distance ↔ upvalue+slot
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **26** | [chapter26 · GC](../chapter26_garbage-collection/) | **Mark-Sweep** · 三色 |
| **27** | [chapter27 · Classes](../chapter27_classes-and-instances/) | **`ObjClass`** · 实例 |
| **28** | Methods | **`this`** · 方法绑定 |

---

## 自测

1. 为何不能 let 闭包长期持有「栈 slot 指针」而不 close？
2. Flattening 时 middle 不捕获 x，inner 会怎样？
3. `ObjClosure` 与 `ObjFunction` 在 `OP_CALL` 路径上的差别？

---

## 阅读进度

- [x] Upvalue / Flatten / Close 结构梳理（本章笔记）
- [ ]  trace `outer()` return 后 inner 读 x 的 upvalue 状态
- [ ] 原书 ch24 闭包 Challenges
