# 第 10 章 · 语义分析（2）静态类型检查 · 速记与自测

← [本章目录](./README.md) · 上一节：[03-static-type-check.md](./03-static-type-check.md)

---

## 本章速记

```text
§1  void成员/重复成员/循环struct · 类型图DFS判环
§2  lvalue/赋值/*/& · DereferenceChecker+try-catch · array/func→ptr
§3  运算限制 · CastNode · 整型提升 · usual conversion · TypeChecker↑
```

---

## 三句背诵

1. **类型定义用图 DFS 抓循环 struct。**
2. **有效性先拦 1=2；catch 防报错泛滥。**
3. **TypeChecker 自底向上，不够就插 CastNode。**

---

## ch9～10 语义五步

| 步 | 章 |
|:--:|-----|
| 1～2 | ch9 |
| 3～5 | **ch10** |

---

## 自测

- [ ] 循环 struct 检测三状态
- [ ] `&1` 为何非法
- [ ] 整型提升与寻常算术转换区别
- [ ] CastNode 在 AST 中的作用

---

## 阅读进度

- [x] ch10 语义分析（2）静态类型检查
- [ ] ch11 中间代码的转换
