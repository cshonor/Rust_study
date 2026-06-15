# 第 27 章 · Classes and Instances（类与实例） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-clox-oop.md](./04-clox-oop.md)

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

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **28** | [chapter28 · Methods](../chapter28_methods/) | 方法 · **`this`** · **`OP_INVOKE`** |
| **13** jlox | Inheritance | **`super`** · clox 后续章 |
| **25** 原书 | Objects | 与实例字段 opcode 重叠阅读 |

---

---

## 自测

1. `Pair()` 在字节码层与 `foo()` 如何区分 callee 类型？
2. 实例字段为何用 Table 而非固定 struct 字段？
3. GC mark 为何要扫描 `instance.fields` 里的 Value？

---

---

## 阅读进度

- [x] Class / Instance / Property 结构梳理（本章笔记）
- [ ] 对照 jlox ch12 画 clox opcode 路径
- [ ] 本章 + 原书 ch25 Challenges
