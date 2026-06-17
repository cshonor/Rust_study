# 3.3 Panics（Panic 与不变式）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

← [06 有效性](./06-validity.md) · 下一节 [08 转换](./08-casting.md)

> **Panic ≠ UB** — panic 是明确定义的终止；[06 Validity](./06-validity.md) 违反才是无保证行为。

维护复杂不变式（如手动改 `Vec` 的 `len` 再逐元素初始化）时，若中间调用可能 **panic** 的用户代码：

- 栈展开可能触发 **`Drop` 于不一致状态**
- 例：`len` 已增大但尾部未初始化 → 析构按「全有效元素」处理 → **二次释放 / 读垃圾**

## 对策

- **`Guard` / defer** 模式
- **`MaybeUninit`** 逐元素提交
- 缩小 panic 可能发生在不一致窗口内的代码

ER → [Item 16](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/README.md)
