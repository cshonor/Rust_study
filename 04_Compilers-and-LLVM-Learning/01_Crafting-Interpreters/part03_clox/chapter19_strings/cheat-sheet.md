# 第 19 章 · Strings（字符串） · 速记与自测

← [本章目录](./README.md) · 上一节：[03-objstring.md](./03-objstring.md)

---

## 本章速记

```text
Obj       type + next · 堆对象基类
继承      ObjString.obj 第一字段 · 指针转换
Value     OBJ_VAL · 字符串走常量池
内存      vm.objects 链表 · freeObjects 退出清理
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **20** | [chapter20 · Hash Tables](../chapter20_hash-tables/) | 开放寻址 · **字符串 intern** |
| **21** | Global Variables | 变量名 = **字符串键** |
| **26** | Garbage Collection | 替换 **freeObjects** |

---

---

## 自测

1. 为什么 `Obj` 必须是 `ObjString` 的第一个字段？
2. 侵入式链表相比「全局指针数组存所有 Obj」有何取舍？
3. 字符串 hash 为什么在创建时就算好？

---

---

## 阅读进度

- [x] Obj / ObjString / 链表内存 结构梳理（本章笔记）
- [ ] 追踪 `copyString` → `emitConstant` 路径
- [ ] 本章 Challenges
