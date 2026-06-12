# 3.1 What Can Go Wrong?（什么会出错）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

**UB（未定义行为）** 不一定立即崩溃：

- 静默错误
- 编译器升级后暴露
- 优化基于错误假设产生荒谬结果

unsafe 边界设计假设：**任何违反 validity / 别名 / 布局 的操作都可能是 UB**。

→ 分项：[06 有效性](./06-validity.md) · [07 Panic](./07-panics.md) · [08 转换](./08-casting.md) · [09 Drop 检查](./09-drop-check.md)
