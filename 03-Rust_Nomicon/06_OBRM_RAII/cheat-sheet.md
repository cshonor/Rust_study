# 速记 · OBRM / RAII · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **Rust 移动 = memcpy，无地址感知构造；Drop 递归清理字段。**
2. **`forget` 泄漏资源但不算内存 UB；代理类型 forget 可能 UAF。**
3. **panic 栈展开须 minimal exception safety；无法恢复则 poison 阻断。**

## 自测

- [ ] 能解释为何纯栈侵入式链表在 Rust 中难安全实现
- [ ] 能说明 `ManuallyDrop` / `Option::take` 打破递归 Drop 的用途
- [ ] 能对比普通泄漏与 `Drain`/`Rc` 代理泄漏的风险差异
- [ ] 能描述 RAII Guard 如何防 double-drop
- [ ] 能解释 `Mutex` poison 的设计意图

## 术语表

| 术语 | 含义 |
|------|------|
| OBRM / RAII | 获取即构造、析构即释放资源 |
| leak amplification | 代理泄漏时放大泄漏以保容器基本安全 |
| minimal exception safety | panic 后仍保持内存安全 |
| Guard / Hole | panic 时由 Drop 恢复中间状态的 RAII 类型 |
| poison | 标记资源处于不可信状态，阻止继续使用 |
