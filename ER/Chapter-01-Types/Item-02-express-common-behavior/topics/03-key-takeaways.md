# Item 2 · 重点结论

← [Item 2 目录](../README.md)

1. **回调 API 优先 `Fn*`，少用裸 `fn`**——让调用方能捕获环境。
2. **参数用「能干活的最宽」`Fn*`**——只调用一次就写 `FnOnce`，别无谓收紧为 `Fn`。
3. **参数优先 Trait bound，少绑死具体 struct**——便于扩展与测试。
4. **无 Trait bound 的泛型 `T` 几乎啥也不能干**——只能 move/drop；要调方法必须 `T: SomeTrait`。

---
