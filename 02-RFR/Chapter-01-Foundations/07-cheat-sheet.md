# 3.3 · 内部可变性 · 速记与自测

← [07 hub](./07-interior-mutability.md) · [05–08 全章速记](./05-08-borrowing-lifetimes-cheat-sheet.md)

---

## 子节速记

```text
07.1  外部 mut vs 内部可变 · 为何需要 · 容器路线图
07.2  UnsafeCell opt-out · Cell/RefCell/Mutex 速查 · RefCell 结构
07.3  Cell=无计数器 · get=拷贝/set=原地覆盖 · RefCell=BorrowFlag
07.4  编译期卡写 vs RefCell 运行时校验 · &self/动态生命周期/细粒度
07.5  对比表 · 误区 · 三句话总纲
```

## 三句话

1. **外部**：编译期看 `mut`。  
2. **内部**：外层 `let`，盒内 `Cell`/`RefCell`。  
3. **`RefCell`**：规则同 `&`/`&mut`，但修改代码能编译、冲突运行时 panic。

## 选型

| 场景 | 用 |
|------|-----|
| 默认 | `let mut` + `&mut` |
| 小 Copy + `&self` | `Cell` |
| `String`/Vec + `&self` | `RefCell` |
| `Rc` 共享写 | `Rc<RefCell<T>>` |
| 多线程 | `Mutex` / `RwLock` |

## 自测

- [ ] 画出 `UnsafeCell` 在容器栈中的位置  
- [ ] `Cell` vs `RefCell` 五维对比  
- [ ] `Rc<RefCell<T>>` 解决哪两个问题？
