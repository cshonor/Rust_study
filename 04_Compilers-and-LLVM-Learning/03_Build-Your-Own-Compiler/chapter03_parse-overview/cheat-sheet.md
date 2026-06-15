# 第 3 章 · 语法分析的概要 · 速记与自测

← [本章目录](./README.md) · 上一节：[03-javacc-overview.md](./03-javacc-overview.md)

---

## 本章速记

```text
§1  词法→语法→语义 · token(字面+种类+值) · Parser→AST/节点
§2  生成器 · LALR(yacc) vs LL(JavaCC) · 可读/无额外库
§3  .jj · javacc→.java · UNICODE_INPUT + UTF-8 Reader
```

---

## 三句背诵

1. **Scanner 出 token，Parser 出 AST。**
2. **本书用 JavaCC（LL），不用 yacc。**
3. **`.jj` → javacc → javac → 进 cbc。**

---

## 对照表

| 术语 | 一句话 |
|------|--------|
| Scanner | 切词 + 分类 + 语义值 |
| Token | 单词三要素打包 |
| AST | 省略纯格式符号的结构树 |
| JavaCC | Java 用 LL 解析器生成器 |

---

## 自测

- [ ] 分析三阶段顺序与产物
- [ ] token 与 AST 节点各是什么
- [ ] LL 选 JavaCC 的原因（说两条）
- [ ] `.jj` 文件主要组成部分

---

## 阅读进度

- [x] ch3 语法分析的概要
- [x] ch4 词法分析
