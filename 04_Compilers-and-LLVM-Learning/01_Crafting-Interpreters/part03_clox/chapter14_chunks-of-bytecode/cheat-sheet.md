# 第 14 章 · Chunks of Bytecode（字节码块） · 速记与自测

← [本章目录](./README.md) · 上一节：[06-chunks-of-bytecode-chunk.md](./06-chunks-of-bytecode-chunk.md)

---

## 本章速记

```text
§14.1  Bytecode：紧凑、可移植、VM 执行
§14.3  Chunk 动态 code[] · OpCode 枚举
§14.4  disassembleChunk 调试
§14.5  常量池 + OP_CONSTANT 单字节索引
§14.6  lines[] 平行数组报行号
```

---

---

## 读后下一步

| 章 | 目录 | 内容 |
|:--:|------|------|
| **15** | [chapter15 · VM](../chapter15_a-virtual-machine/) | **`ip`** · **`run()`** · 值栈 · 算术 |
| **16** | [chapter16 · Scanning](../chapter16_scanning-on-demand/) | 按需 Token |
| **17** | Compiling Expressions | Token → **编译进 Chunk** |

---

---

## 自测

1. 为什么 `1.2` 不直接编码进 `OP_ADD` 之类的指令里？
2. `lines[]` 为何按**字节**而非按「指令」索引？
3. 反汇编器在 clox 开发流程里相当于 jlox 的什么工具？

---

---

## 阅读进度

- [x] §14.1、§14.3～§14.6 结构梳理（本章笔记）
- [ ] 本地 clox：跑 disassemble 示例 Chunk
- [ ] 本章 Challenges
