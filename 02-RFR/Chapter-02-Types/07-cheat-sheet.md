# 2.3 · 相干性 · 孤儿规则 · 速记

← [07 hub](./07-coherence-orphan-rule.md) · [orphan-rule-demo](./orphan-rule-demo/) · [章索引](./README.md)

---

## 三句话

1. **trait 与 type 至少一方本地。**  
2. **Coverage：`impl From<MyLocal> for Foreign`。**  
3. **双外部 → NewType + `Deref`。**

## Newtype 四用途

类型安全 · 绕孤儿 · 校验构造 · 专属方法

→ 详 [07.3](./07-3-newtype-practice.md)

## 自测

- [ ] 为何不能 `impl Display for Vec<u8>`？  
- [ ] blanket `impl<T: Debug> MyTrait for T` 有何风险？  
- [ ] NewType 有运行时开销吗？
