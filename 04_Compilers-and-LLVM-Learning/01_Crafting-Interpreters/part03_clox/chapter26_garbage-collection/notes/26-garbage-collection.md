# 第 26 章 · Garbage Collection（垃圾回收）

> 在线：[garbage-collection.html](https://craftinginterpreters.com/garbage-collection.html) · 中文：[第 26 章 垃圾回收](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../../README.md) · [本书目录](../../../本书目录.md) · 上一章：[ch25 Closures](../chapter25_objects/notes/25-closures.md)

---

## 本章定位

ch19 起 **堆分配**（字符串、函数、闭包、Upvalue…）仅靠 **进程退出 `freeObjects`** 不够 → 手写 **Mark-Sweep GC**，替换 ch19 的朴素释放。

| 对比 | ch19 | ch26 |
|------|------|------|
| 释放 | **`freeObjects()` 退出时** | **可达性 GC** |
| 追踪 | 侵入式 **`vm.objects` 链表** | 同链表 + **mark 位** |
| 字符串表 | intern 表 | **弱引用** 特殊清扫 |

| 主题 | 要点 |
|------|------|
| **Mark** | 根集合 · **三色抽象** |
| **Sweep** | 白对象 **free** |
| **Weak refs** | **`vm.strings`** 与 GC 协同 |
| **触发** | **自适应阈值** |

---

## 标记阶段与三色抽象（Marking & Tri-color Abstraction）

**根 (Roots)**：GC 起点 — 凡 **mutator 直接可达** 之处：

| 根 | 示例 |
|----|------|
| **栈** | `vm.stack` 上所有 Value |
| **全局** | **`vm.globals`** |
| **CallFrame** | closure / function 引用 |
| **Upvalue** | open/closed 链 |
| **编译期常量** | 当前 chunk 常量池（实现随版本） |

**三色**（概念模型）：

| 色 | 含义 |
|----|------|
| **白** | 未访问 · sweep 时若仍白 → **垃圾** |
| **灰** | 已发现 · **引用尚未全部扫描** · 在 **工作栈/列表** |
| **黑** | 自身及 **直接引用对象** 均已标记 |

**算法（DFS / 工作栈）**：

```text
markRoots() → 所有根标灰并入 work
while work not empty:
  obj = pop(work)
  mark 其引用的 Obj（子对象）
  obj → 黑
```

**循环引用**：对象图 **有环** 时，仅从根 DFS + **visited/mark 位** 避免死循环；不可达环整体仍白 → 被回收。

**实现**：`Obj` 上 **`isMarked` 位**（或深色标记）；`markObject` / `markValue` / `markTable` 递归。

**对照 RFR / 系统**：Rust **所有权** 编译期证明；clox **运行时 tracing GC** — 与 JVM、Go 同族。

---

## 清除阶段（Sweeping）

**Mark 完成后**：

```text
for obj in vm.objects 链表:
  if !obj.isMarked:
    free(obj)           // 白 → 垃圾
  else:
    obj.isMarked = false  // 清除 mark 位，下轮 GC 用
```

| 要点 | 说明 |
|------|------|
| **Sweep 线性扫堆链表** | 与 allocate 时链接的 **`vm.objects`** 一致 |
| **黑色/存活** | 保留并 **unmark** |
| **白色** | **`reallocate(..., 0)` 释放** |

**碎片**：Mark-Sweep **不移动对象**（非 compacting）；简单，适合教学 VM。

---

## 弱引用与字符串池（Weak References）

**问题**：**`vm.strings`** intern 表 **强引用** 字符串 → 程序中已无用的字符串 **仍被表钉住** → 假存活；若表不更新，还可能 **悬空**（若别处误 free）。

**处理**（书中策略概要）：

| 阶段 | 对 intern 表 |
|------|--------------|
| **Mark 后 / Sweep 前** | 表作为 **弱引用**：**移除** 未被 mark 的字符串条目 |
| **Sweep** | 只 free **堆链表上未 mark 的 Obj** |

**效果**：intern 表 **不阻止** 字符串被回收；相等 intern 语义在 **存活期** 内仍有效。

**对照 ch20 [intern](../chapter20_hash-tables/notes/20-hash-tables.md)**：性能优化与 GC **必须协同设计**。

---

## 触发回收（When to Collect）

**不能** 每次 `allocate` 都 GC（太慢）→ **字节阈值** + **自适应**。

| 机制 | 说明 |
|------|------|
| **`bytesAllocated`** | 累计分配量 |
| **`nextGC`** | 下次触发阈值 |
| **`collectGarbage()`** | 超阈值 → mark + sweep |
| **自适应** | GC 后根据 **存活 bytes** 调整 **`nextGC`**（如 **`alive * GC_HEAP_GROW_FACTOR`**） |

**平衡**：存活对象多 → 阈值升高，减少 GC 频率；存活少 → 更勤回收，控内存。

**REPL / 长运行**：ch19 退出才 free → ch26 后 **可持续交互**。

---

## GC 与 clox 子系统

```text
         Roots
    stack · globals · frames · upvalues
              │
              ▼ mark (tri-color / work stack)
         所有可达 Obj 变黑
              │
    sweep vm.objects ──► free 白对象
              │
    清理 vm.strings 弱引用条目
              │
         重置 nextGC
```

---

## 本章速记

```text
Mark      根 → 三色/work 栈 → markObject 图遍历
Sweep     链表扫未 mark → free · 清除 mark 位
Strings   intern 表弱引用 · sweep 间清理
Trigger   bytesAllocated vs nextGC · 自适应阈值
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **27** | [chapter27 · Classes](../chapter27_classes-and-instances/) | **`OP_CLASS`** · **`ObjInstance`** |
| **28** | Methods | 方法 · **`this`** |
| **19** ch19 | Strings | 对比 exit-time free → GC |

---

## 自测

1. 闭包循环引用两个 Obj，无根可达时 GC 能否回收？
2. 为何 intern 表不能对字符串做普通强 mark？
3. `nextGC` 自适应调大有什么好处与风险？

---

## 阅读进度

- [x] Mark-Sweep / 三色 / 弱引用 / 触发 结构梳理（本章笔记）
- [ ] 画一轮 GC 前后 objects 链表与 strings 表
- [ ] 本章 Challenges
