# 第 6 章 · 过程抽象 · 速记与自测

← [本章目录](./README.md) · 上一节：[06-memory-and-gc.md](./06-memory-and-gc.md)

---

## 本章速记

```text
§1  过程=控制抽象 · 词法作用域 · OOP=record+dispatch
§2  AR/栈帧: 局部/参数/返回址/保存寄存器/链指针 · 栈LIFO
§3  局部=FP+offset · 嵌套用static link或display
§4  值传递vs引用 · 返回值ABI槽位
§5  precall→prologue→epilogue→postreturn · SysV/MS ABI
§6  代码/静态/堆/栈 · first-fit · RC/mark-sweep/copying GC
```

---

## 三句背诵

1. **每次调用一个 AR，栈上 LIFO 管理。**
2. **传参/返回/保存寄存器靠链接约定四段序列。**
3. **栈管调用边界，堆管不确定生命周期；GC 或手动释放在此分野。**

---

## 与 CI / Rust 对照

| 橡书 ch6 | 本仓库 |
|----------|--------|
| CallFrame | clox ch24 |
| Upvalue/闭包 | clox ch25 |
| Mark-sweep GC | clox ch26 |
| ABI / 无 GC | Rust · RFR ch1/13 |

---

## 自测

- [ ] AR 里至少 4 种字段
- [ ] static link vs display 各解决什么
- [ ] call-by-value vs by-reference
- [ ] precall / prologue / epilogue / postreturn 各谁执行
- [ ] 三种 GC 算法各一句
- [ ] 栈分配 vs 堆分配对 HFT 意味着什么

---

## 阅读进度

- [x] ch6 过程抽象（本章笔记）
- [ ] ch7 代码形态
