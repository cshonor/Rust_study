# 第 20 章 · Hash Tables（哈希表） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-ast.md](./04-ast.md)

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

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **21** | [chapter21 · Global Variables](../chapter21_global-variables/) | **`vm.globals`** · DEFINE/GET/SET |
| **22** | Local Variables | 栈 slot · 非全局表 |
| **25** | Objects | 实例字段表 |

---

---

## 自测

1. 线性探测聚集（primary clustering）是什么？本书为何仍选它？
2. 负载因子超过 75% 不扩容会怎样？
3. 没有 intern，`OP_EQUAL` 对两个内容相同的字符串要做什么？

---

---

## 阅读进度

- [x] 开放寻址 / 墓碑 / intern 结构梳理（本章笔记）
- [ ] 手工模拟插入冲突与墓碑删除
- [ ] 本章 Challenges
