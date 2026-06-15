# 第 21 章 · Global Variables（全局变量） · 速记与自测

← [本章目录](./README.md) · 上一节：[05-ast.md](./05-ast.md)

---

## 本章速记

```text
语句    表达式 stmt 末尾 OP_POP · print → OP_PRINT
声明    OP_DEFINE_GLOBAL · 名入常量池 intern
读写    OP_GET/SET_GLOBAL · vm.globals 表
赋值    canAssign 控制 · 仅 ASSIGNMENT 级可 =
错误    未定义全局 · runtimeError + line
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **22** | [chapter22 · Local Variables](../chapter22_local-variables/) | 栈 slot · **`OP_GET/SET_LOCAL`** |
| **23** | Jumping Back and Forth | 控制流 bytecode |
| **11** jlox | Resolver | distance · 闭包静态绑定（clox ch22+ 对应） |

---

---

## 自测

1. 为什么 `SET_GLOBAL` 执行后还要把值 push 回栈？
2. `canAssign=false` 时遇到 `foo = 1` 会怎样？
3. 全局变量名为何放在常量池而不是指令里嵌字符串？

---

---

## 阅读进度

- [x] 语句 / 全局 var / GET·SET / canAssign 结构梳理（本章笔记）
- [ ] 对照 jlox ch8 Environment 与 clox globals 表
- [ ] 本章 Challenges
