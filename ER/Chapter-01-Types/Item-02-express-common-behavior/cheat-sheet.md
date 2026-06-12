# Item 2 · 记忆卡片

← [Item 2 目录](./README.md)

| 要点 | 一句 |
|------|------|
| `self` 三种 | `&` 读、`&mut` 改、`self` 拿走 → [09](./09-methods-and-self.md) |
| `fn` vs 闭包 | `fn` 无捕获；闭包有唯一类型 + `Fn*` |
| `Fn*` 选宽 | 能 `FnOnce` 就别写死 `Fn`；Fn > FnMut > FnOnce 能力 |
| 逻辑链 | 函数→方法→fn→闭包→Trait→静/动态 → [02](./02-logic-flow.md) |
| `Fn*` 兼容 | 要求 FnOnce 最宽；call_once 会消耗闭包 |
| 静/动态 | 静态=编译硬编码地址；dyn=运行查 vtable → Item 12 §06 |
| `FnOnce<()>` | `<>` 里是**入参元组**，不是返回值；`()` = 无入参 |
| `<>` 两种 | `'a` 管生命周期；`(T,)` 管调用签名 → [06](./06-trait-generic-params.md) |
| 两种 `'a` | 顶层 `Trait<'a>` vs 元组内 `&'a` → [07](./07-lifetime-vs-type-in-angle-brackets.md) |
| `'env` | **Scope 环境**，非 trait 内引用 → [08](./08-scope-env-lifetime.md) |
| 泛型 | 无 bound 的 `T` 几乎只能 move |
| `dyn Trait` | 要对象安全；否则用泛型 |
