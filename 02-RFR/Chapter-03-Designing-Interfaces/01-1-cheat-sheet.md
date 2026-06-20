# with_ / Builder · 速记

← [01-1 详细例](./01-1-with-series-and-builder.md) · [01 命名](./01-naming-practices.md)

---

## 核心

方法链替代多参数构造 · 可读 · 可选字段只写需要的

## Vec::with_capacity

`new()` → push 扩容反复拷贝 · `with_capacity(N)` → 一次预留 · **容量≠长度**

## reqwest 链

`Client::new().get(url).header(...).timeout(...).send()` → 中间 RequestBuilder → send 消耗

## 手写 Builder

`new` 默认值 · `with_* (mut self) -> Self` · `build(self)` 产出

## vs 其它

`with_` 构造 · `as_` 借 · `into_` 消耗

## 自测

- [ ] `with_capacity(100)` 后 `len()` 是多少？  
- [ ] 为何 100 万次 push 用 `new()` 更慢？  
- [ ] `.send()` 前 RequestBuilder 发请求了吗？
