# 第 18 章 · Types of Values（值的类型） · 速记与自测

← [本章目录](./README.md) · 上一节：[04-runtime-errors.md](./04-runtime-errors.md)

---

## 本章速记

```text
Value     type tag + union · IS_/AS_/_VAL 宏
Falsiness nil/false 假 · OP_NOT
Compare   <= >= 脱糖为 GREATER + NOT
Runtime   checkNumber · runtimeError + line
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **19** | [chapter19 · Strings](../chapter19_strings/) | **`Obj`** · **`ObjString`** |
| **20** | Hash Tables | 开放寻址 · **intern** |
| **22** | Local Variables | 局部 slot · 非仅全局 |

---

---

## 自测

1. 为什么 `0` 在 Lox 里是 truthy？
2. `<=` 脱糖后 VM 需要几条指令？比单独 `OP_LESS_EQUAL` 多什么？
3. Tagged union 与 jlox 的 `Object` 各适合什么运行时？

---

---

## 阅读进度

- [x] Tagged union / falsiness / 比较 / 运行时错误 结构梳理（本章笔记）
- [ ] 列出 `ValueType` 扩展路径（string/obj）
- [ ] 本章 Challenges
