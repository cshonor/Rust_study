# 第 19 章 · Strings（字符串）

> 在线：[strings.html](https://craftinginterpreters.com/strings.html) · 中文：[第 19 章 字符串](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch18 Types of Values](../chapter18_types-of-values/18-types-of-values.md)

---

## 本章定位

bool / number / nil 可 **按值** 塞进 `Value`；**字符串长度可变、堆分配** → 引入 **Obj 对象系统**，为 GC（ch26）与哈希表键（ch20）铺路。

| 对比 | 立即数 Value | Obj |
|------|--------------|-----|
| 存储 | 栈 / 常量池内联 | **堆** |
| 大小 | 固定 | **可变长** |
| 生命周期 | 编译期 / 栈帧 | 需 **追踪与释放** |

| 主题 | 要点 |
|------|------|
| **Obj / 结构体继承** | 首字段 **`Obj`** · 统一转换 |
| **ObjString** | 字符数组 + length + hash |
| **内存管理** | **`vm.objects` 侵入式链表 · 退出时 `freeObjects`** |

---

## 对象与结构体继承（Values and Objects）

**堆上值 = 对象 (Obj)**：

```c
struct Obj {
  ObjType type;
  struct Obj* next;   // 链表节点
};
```

**具体类型**（结构体继承技巧）：

```c
struct ObjString {
  Obj obj;      // 必须是第一个字段
  int length;
  char* chars;
  uint32_t hash;
};
```

| 技巧 | 说明 |
|------|------|
| **`Obj` 为首字段** | `(ObjString*)` ↔ `(Obj*)` **安全 upcast/downcast** |
| **`ObjType` 枚举** | `OBJ_STRING` 等，运行时判别 |
| **模拟 OOP** | C 无 class · **组合 + 首字段布局** |

**`Value` 扩展**：增加 **`OBJ_VAL(obj*)`** · **`IS_OBJ`** · **`AS_STRING`** 等。

**编译字符串字面量**：

- Scanner 已识别 **`TOKEN_STRING`** lexeme。
- Compiler **`copyString`** → 堆上 **`ObjString*`** → **`emitConstant(OBJ_VAL(string))`**。

---

## 内存管理基础（Memory Management）

当前阶段 **无 GC** → **追踪所有分配**，进程结束 **一次性释放**。

```text
allocateObject(size, type):
  obj = reallocate(...)
  obj->next = vm.objects
  vm.objects = obj

freeObjects():
  walk vm.objects 链表 → free 每个 ObjString 等
```

| 设计 | 原因 |
|------|------|
| **侵入式链表 `next`** | 每个 Obj 自带链表指针 · 无单独容器 |
| **退出时释放** | ch26 前足够；泄漏仅存在于 REPL 长会话 |
| **为 GC 铺垫** | 遍历 **`vm.objects`** = 未来 **mark 根集合** 的起点之一 |

**对照 RFR / 系统编程**：堆分配 · 所有权；clox 最终由 **GC** 接管，而非 Rust 式静态所有权。

---

## ObjString 要点

| 字段 | 作用 |
|------|------|
| **`length`** | 非 NUL 依赖长度（可含 `\0`） |
| **`chars`** | 堆缓冲区，通常 **`length+1`** 结尾 `\0` 便于 `printf` |
| **`hash`** | 预计算 **FNV-1a**（ch20 表键 · intern 用） |

**运行时**：字符串 **拼接、打印** 等在后续章节扩展；本章重点是 **表示 + 分配 + 链表**。

---

## 本章速记

```text
Obj       type + next · 堆对象基类
继承      ObjString.obj 第一字段 · 指针转换
Value     OBJ_VAL · 字符串走常量池
内存      vm.objects 链表 · freeObjects 退出清理
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **20** | [chapter20 · Hash Tables](../chapter20_hash-tables/) | 开放寻址 · **字符串 intern** |
| **21** | Global Variables | 变量名 = **字符串键** |
| **26** | Garbage Collection | 替换 **freeObjects** |

---

## 自测

1. 为什么 `Obj` 必须是 `ObjString` 的第一个字段？
2. 侵入式链表相比「全局指针数组存所有 Obj」有何取舍？
3. 字符串 hash 为什么在创建时就算好？

---

## 阅读进度

- [x] Obj / ObjString / 链表内存 结构梳理（本章笔记）
- [ ] 追踪 `copyString` → `emitConstant` 路径
- [ ] 本章 Challenges
