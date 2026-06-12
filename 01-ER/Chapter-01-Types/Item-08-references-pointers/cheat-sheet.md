# Item 8 · 记忆卡片

← [Item 8 目录](./README.md)

| 要点 | 一句 |
|------|------|
| `&T` | 8B 瘦指针；借用监督 |
| 胖指针 | 切片/str、dyn = ptr + 元数据 |
| `Box` | 堆 + 独占 |
| `Rc/Arc` | 共享所有权；环用 `Weak` |
| `RefCell` | 内部可变；违反规则 runtime panic |
| 裸指针 | 仅 unsafe / FFI |
