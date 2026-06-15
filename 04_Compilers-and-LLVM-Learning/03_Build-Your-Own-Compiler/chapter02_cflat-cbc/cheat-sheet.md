# 第 2 章 · C♭ 和 cbc · 速记与自测

← [本章目录](./README.md) · 上一节：[03-compiler-control-flow.md](./03-compiler-control-flow.md)

---

## 本章速记

```text
§1  C♭：C子集+指针 · 无cpp/浮点/enum/const/volatile · import 替 #include
§2  Java5 · 11包 · 数据(ast/ir/type) vs 处理(compiler/parser)
§3  build: compile×N → assemble → link
     compile: parse → sema → IR → asm
```

---

## 三句背诵

1. **C♭ 为写编译器而简化的 C，用 import 引库。**
2. **cbc 分包存 AST/IR/类型，compiler 包总控。**
3. **compile 四步 = ch1 狭义编译内四阶段。**

---

## 对照表

| 符号 | 含义 |
|------|------|
| C♭ | 源语言 |
| cbc | 编译器实现 |
| `Compiler.build` | 多文件到 ELF |
| `compile` | 单文件到汇编 |

---

## 自测

- [ ] C♭ 删掉预处理器后如何 include 库
- [ ] 11 包两大分类
- [ ] 画出 `build` 与 `compile` 调用关系
- [ ] AST/IR 分别在哪个包

---

## 阅读进度

- [x] ch2 C♭ 和 cbc
- [ ] ch3 语法分析的概要（第1部分开始）
