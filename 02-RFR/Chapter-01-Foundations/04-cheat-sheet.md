# 速记 · Ownership · 自测

← [04 所有权索引](./04-ownership.md)

---

## 三句背诵

1. **三条规则：唯一所有者 · 出作用域 drop · 非 Copy 则 move。**
2. **Move/Copy 看 `Copy` trait，不看栈/堆；要两份堆 → `.clone()`。**
3. **局部变量 drop 逆声明；struct 字段 drop 正定义 — 方向相反。**

## 自测

- [ ] 能解释栈上 `Point` 为何默认仍是 Move
- [ ] 能说出 `String` move 时堆上只有一份数据
- [ ] 能背出局部变量 vs struct 字段的 drop 顺序
- [ ] 能说明 `&T` 为何不 drop 目标
- [ ] 能区分 unwind 与 `panic=abort` 下的 Drop

## 术语表

| 术语 | 含义 |
|------|------|
| RAII | 获取资源即初始化；出作用域自动释放 |
| Move | 转移所有权；原绑定失效 |
| Copy | 隐式按位复制；原绑定仍有效 |
| Clone | 显式复制；可深拷贝堆 |
