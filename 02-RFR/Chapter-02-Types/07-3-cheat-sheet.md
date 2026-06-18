# 2.3.3 · Newtype · 速记

← [07.3 Newtype 详解](./07-3-newtype-practice.md) · [07 hub](./07-coherence-orphan-rule.md) · [章索引](./README.md)

---

## 定义

单字段元组 struct · 编译期新类型 · **零开销** · `repr(transparent)` 保布局

## 四大用途

1. **类型安全** — `UserId` vs `OrderId`  
2. **绕孤儿规则** — `MyVec(Vec<T>)` + `impl Display`  
3. **校验构造** — 私有 + `new()`  
4. **专属方法** — `Price::fee()`

## vs `type` 别名

| 别名 | Newtype |
|------|---------|
| 同一类型 | 新类型 |
| 无隔离 | 强隔离 |
| 不能绕孤儿 | 可以 |

## 双外部

本地 trait **或** 本地 type **或** NewType

## Deref

便捷取值 · **勿滥用**（弱化安全）

## 自测

- [ ] 为何 `type Uid = u32` 不能防传参反了？  
- [ ] 给 `Vec<i32>` 写 `Display` 为何非法，Newtype 如何合法？  
- [ ] Newtype 有运行时开销吗？
