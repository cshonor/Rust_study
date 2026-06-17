# 2.5 · 标记 Trait · 速记

← [09 hub](./09-marker-traits.md) · [章索引](./README.md)

---

## 五大标记

| Trait | 一句话 |
|-------|--------|
| `Sized` | 编译期固定 `size_of`；`?Sized` 放开 DST |
| `Copy` | 位拷贝，原值仍有效 |
| `Send` | 所有权可移到另一线程 |
| `Sync` | `&T` 可跨线程共享（`&T: Send`） |
| `Unpin` | 可安全移动（非 Pin 钉住） |

## 三句话

1. **`Sized`：默认要；`?Sized` 给 DST。**  
2. **`Copy` 隐式位拷贝；`Send` 移线程；`Sync` 共享 `&T`。**  
3. **`Send`/`Sync` 手动 impl 必须 `unsafe` + 孤儿规则。**
