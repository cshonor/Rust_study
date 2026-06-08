# Item 2 · 记忆卡片

← [Item 2 目录](../README.md)

| 要点 | 一句 |
|------|------|
| `self` 三种 | `&` 读、`&mut` 改、`self` 拿走 |
| `fn` vs 闭包 | `fn` 无捕获；闭包有唯一类型 + `Fn*` |
| `Fn*` 选宽 | 能 `FnOnce` 就别写死 `Fn` |
| 泛型 | 无 bound 的 `T` 几乎只能 move |
| `dyn Trait` | 要对象安全；否则用泛型 |
