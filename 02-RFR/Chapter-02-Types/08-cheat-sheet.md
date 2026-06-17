# 2.4 · Trait 限定 · 速记

← [08 hub](./08-trait-bounds.md) · [05 分发速记](./05-cheat-sheet.md) · [章索引](./README.md)

---

## 子节速记

```text
08.1  <T: Trait> · impl Trait · &dyn Trait 三分法
08.2  for<'a> HRTB · 回调吃任意生命周期 &str
08.3  双 impl 陷阱 · demo · 避坑表
```

## 三句话

1. **`T` / `impl` → 静态；`dyn` → vtable。**  
2. **两参数要同类型 → `<T>`，不要双 `impl`。**  
3. **`F: for<'a> Fn(&'a str)` — 回调标准写法。**

## 自测

- [ ] 对比 `fn f(x: impl Display)` 与 `fn f(x: &dyn Display)`  
- [ ] 写出 HRTB 版「接收任意 `&str` 闭包」签名  
- [ ] 为何 `Vec<Box<dyn Error>>` 需要 object-safe trait？
