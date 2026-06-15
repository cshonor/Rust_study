# 第 5 章 · 基于 JavaCC 的解析器的描述 · 速记与自测

← [本章目录](./README.md) · 上一节：[02-ambiguity-and-lookahead.md](./02-ambiguity-and-lookahead.md)

---

## 本章速记

```text
§1  Parser：token→语句/表达式/树 · 终端 vs 非终端 · EBNF * + | []
§2  二义性 · Choice conflict · 提公因子 · LOOKAHEAD(n/规则) · else
```

---

## 三句背诵

1. **终端 = token/字面；非终端 = stmt()/expr() 规则。**
2. **`|` 同首 → 选择冲突；默认只看 1 token。**
3. **LOOKAHEAD 消冲突；dangling else 用 LOOKAHEAD(1) 绑内层 if。**

---

## EBNF 对照

| 符号 | 含义 |
|------|------|
| 并排 | 连接 |
| `*` | 0+ |
| `+` | 1+ |
| `\|` | 选择 |
| `[]` | 可选 |

---

## 自测

- [ ] Parser 相对 Scanner 多做什么
- [ ] 写一条含 `|` 和 `[]` 的概念产生式
- [ ] 解释 dangling else 与 LOOKAHEAD(1) 的关系
- [ ] LOOKAHEAD 填数字 vs 填规则

---

## 阅读进度

- [x] ch5 基于 JavaCC 的解析器的描述
- [x] ch6 语法分析
