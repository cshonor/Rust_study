# 速记 · Uninitialized Memory · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **Safe 用分支分析阻止读未初始化栈变量；move-out 后原变量逻辑未初始化。**
2. **Drop flags 跟踪「是否该 Drop」；复杂路径才需运行时 flag。**
3. **MaybeUninit 不 Drop 内部；ptr::write 盲写绕过 Drop — 新代码勿用 `uninitialized`。**

## 自测

- [ ] 能解释条件初始化为何需要全路径赋值
- [ ] 能说明 drop flags 何时有运行时开销
- [ ] 能对比 `MaybeUninit` 与普通变量的 Drop 行为
- [ ] 能说出 `ptr::write` / `copy` / `copy_nonoverlapping` 的分工
- [ ] 能描述 Vec 增长如何组合本章 API（见 [04-vec-prelude.md](./04-vec-prelude.md)）

## 术语表

| 术语 | 含义 |
|------|------|
| 逻辑未初始化 | move-out 后变量槽位不可再读 |
| drop flag | 栈上跟踪值是否已初始化、是否应 Drop |
| MaybeUninit | 合法标记「可能未初始化」的包装类型 |
| assume_init | 断言已初始化后转为 `T`（误用即 UB） |
| ptr::write | 向可能未初始化地址写入，不 Drop 旧值 |
