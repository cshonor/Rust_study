# 第 20 章 · Hash Tables（哈希表）

> 在线：[hash-tables.html](https://craftinginterpreters.com/hash-tables.html) · 中文：[第 20 章 哈希表](https://craftinginterpreters-zh-jet.vercel.app/)  
> 所属：Part III · [part03_clox](../README.md) · [本书目录](../../本书目录.md) · 上一章：[ch19 Strings](../chapter19_strings/19-strings.md)

---

## 本章定位

动态语言的核心基础设施：**全局变量、属性、方法表** 都依赖 **O(1) 均摊** 查找。clox **手写 C 哈希表**，并为 **字符串驻留 (interning)** 服务。

| 用途（本章及后） | 结构 |
|------------------|------|
| **字符串 intern** | 全局 **`vm.strings`** 表 |
| **全局变量 ch21** | **`vm.globals`** |
| 实例字段 / 方法 | ch25+ |

| 主题 | 要点 |
|------|------|
| **开放寻址 + 线性探测** | 无链表 · 缓存友好 |
| **FNV-1a + 负载 75%** | 扩容 **rehash** |
| **Tombstones** | 删除不破坏探测链 |
| **String Interning** | 相等 = **指针比较** |

---

## 开放寻址与线性探测（Open Addressing and Linear Probing）

**不用** 分离链接法（bucket + 链表）：

| 开放寻址 | 说明 |
|----------|------|
| 存储 | **单数组 `entries[]`** |
| 冲突 | **线性探测**：`index = (index + 1) % capacity` 找下一空位 |
| 缓存 | 连续内存 · **CPU cache line** 友好 |

**Entry 状态**：

| 状态 | 含义 |
|------|------|
| **空 (NULL key)** | 从未使用 |
| **占用** | 有效键值 |
| **墓碑 (Tombstone)** | 已删 · 查找跳过 · **插入可复用** |

**为何墓碑？**

- 直接清空槽位 → 打断探测链 → **后续键「找不到」**。
- 墓碑标记「此处曾有键，请继续探测」。

---

## 哈希函数与负载因子（Hash Functions and Load Factor）

| 项 | 选择 |
|----|------|
| **哈希** | **FNV-1a**（字符串 → **uint32_t**） |
| **负载因子** | **entries / capacity > 0.75** → **grow** |
| **扩容** | 更大数组 · **所有 Entry 重新插入（rehash）** |

```text
count / capacity > 0.75
  → capacity *= 2
  → 新表 rehash 所有 live entries
```

**均摊 O(1)**：单次扩容贵，插入均摊便宜。

**键类型**：本章以 **`ObjString*`** 为键（已含 **预计算 hash**）。

---

## 字符串驻留（String Interning）

**问题**：哈希表频繁 **`strcmp`** 很慢。

**驻留**：全局表 **`tableSet(vm.strings, string, NULL)`**（或等价 API）：

| 规则 | 效果 |
|------|------|
| 创建字符串前 **查表** | 已存在 → **返回已有实例** |
| 不存在 | 分配新 **`ObjString`** 并插入 |
| 相等 | **`a == b`** ⇔ **指针相同**（同内容必同址） |

**编译器 / 运行时**：

- 变量名、属性名 → **intern 后的 `ObjString*`**。
- **`OP_EQUAL`** 对字符串可 **先比指针**（类型相同时）。

**对照**：Java **String pool** · Python **intern** · Lua string table。

---

## Table API（概念）

| 操作 | 行为 |
|------|------|
| **`tableSet`** | 插入/更新 · 触发扩容 |
| **`tableGet`** | 线性探测查找 |
| **`tableDelete`** | 置 **墓碑** |
| **`tableFindString`** | intern 专用查找 |

---

## 本章速记

```text
结构    开放寻址 · 线性探测 · Entry 数组
冲突    探测下一槽 · 75% 扩容 rehash
删除    Tombstone 保链
Intern  vm.strings · 同文同址 · 指针相等
Hash    FNV-1a · ObjString.hash 缓存
```

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **21** | [chapter21 · Global Variables](../chapter21_global-variables/) | **`vm.globals`** · DEFINE/GET/SET |
| **22** | Local Variables | 栈 slot · 非全局表 |
| **25** | Objects | 实例字段表 |

---

## 自测

1. 线性探测聚集（primary clustering）是什么？本书为何仍选它？
2. 负载因子超过 75% 不扩容会怎样？
3. 没有 intern，`OP_EQUAL` 对两个内容相同的字符串要做什么？

---

## 阅读进度

- [x] 开放寻址 / 墓碑 / intern 结构梳理（本章笔记）
- [ ] 手工模拟插入冲突与墓碑删除
- [ ] 本章 Challenges
