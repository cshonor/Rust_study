# 第 5 章 · 中间表示 · 速记与自测

← [本章目录](./README.md) · 上一节：[07-symbol-tables.md](./07-symbol-tables.md)

---

## 本章速记

```text
§1  IR=前后端桥梁 · m×n · 优化基础
§2  图IR / 线IR / 混合(CFG块+块内线)
§3  AST(嵌套) · DAG(共享子expr) · CFG(基本块+边)
§4  栈机(紧凑/clox/JVM) · 三地址/ILOC(分析/regalloc)
§5  SSA: 每名一次赋值 · φ在合流点 · 简化数据流
§6  reg-to-reg(现代) vs mem-to-mem(慢) · 虚拟寄存器
§7  符号表: 类型/作用域/存储 · 散列+嵌套scope
```

---

## 三句背诵

1. **IR 解耦源与目标，是优化的主战场。**
2. **AST 看结构，CFG 看控制，三地址/SSA 看数据流。**
3. **SSA + φ 让 def-use 清晰；符号表管源名，IR 名管临时值。**

---

## 与 CI / LLVM 对照

| 橡书 ch5 | 本仓库 |
|----------|--------|
| AST | jlox |
| 栈线性 IR | clox 字节码 |
| SSA | LLVM IR · ch8～10 |
| 符号表 | jlox ch11 |

---

## 自测

- [ ] m×n 架构什么意思
- [ ] AST vs DAG vs CFG 各表达什么
- [ ] 栈机 vs 三地址 各适合什么
- [ ] SSA 规则 + φ 何时出现
- [ ] reg-to-reg 为何是现代默认
- [ ] 符号表如何处理作用域遮蔽

---

## 阅读进度

- [x] ch5 中间表示（本章笔记）
- [ ] ch6 过程抽象
