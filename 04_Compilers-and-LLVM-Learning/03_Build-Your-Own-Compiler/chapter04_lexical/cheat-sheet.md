# 第 4 章 · 词法分析 · 速记与自测

← [本章目录](./README.md) · 上一节：[04-structured-lexing-states.md](./04-structured-lexing-states.md)

---

## 本章速记

```text
§1  JavaCC 正则：字符组 · *+?{} · |
§2  TOKEN · 最长匹配/同长先定义 · 二/八/十六进制
§3  SKIP / SPECIAL_TOKEN · 空白与 //
§4  状态迁移 · MORE · 块注释/字符串/字符
```

---

## 三句背诵

1. **TOKEN 出词，SKIP 静音。**
2. **保留字靠最长匹配 + 规则顺序。**
3. **有结构的词法用状态 + MORE，不能单靠一条贪婪正则。**

---

## 命令对照

| 命令 | 效果 |
|------|------|
| TOKEN | 生成 token |
| SKIP | 丢弃，无 token |
| SPECIAL_TOKEN | 跳过但可链式访问 |
| MORE | 同一 token 继续累加 |

---

## 自测

- [ ] JavaCC 正则五种构造各举一例
- [ ] 最长匹配与同长优先
- [ ] SKIP vs SPECIAL_TOKEN
- [ ] 块注释状态机草图（DEFAULT ↔ IN_COMMENT）

---

## 阅读进度

- [x] ch4 词法分析
- [ ] ch5 基于 JavaCC 的解析器的描述
