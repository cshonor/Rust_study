# 速记 · `&mut T` vs 内部可变性

← [06 可变引用](./06-mutable-references.md) · [07 内部可变性](./07-interior-mutability.md) · [05 共享引用](./05-shared-references.md)

---

## 三句背诵

1. **`&mut` = 编译期独占，零运行时成本；内部可变 = 外表 `&T`，里面受控改。**
2. **底层都是别名规则：`&mut` 靠编译器，`RefCell` 靠运行时计数，`Mutex` 靠锁。**
3. **`UnsafeCell` 关掉 `&T` 的「永不修改」假设 — 否则 LLVM 会错优化。**

## 一张表选对工具

| 需求 | 选用 |
|------|------|
| 单线程、清晰独占修改 | `&mut T` 或 `let mut` |
| 单线程、多个 `&` 句柄、要改内部 | `RefCell<T>` |
| 单线程、`Copy` 小值、不要内部引用 | `Cell<T>` |
| 多线程共享修改 | `Mutex<T>` / `RwLock<T>` |
| 只要读、多人并发读 | `Arc<T>` + 内部容器，或 `RwLock` 读锁 |

## `&T` / `&mut T` / 内部可变 对比

| | `&T` | `&mut T` | `RefCell`（外表 `&RefCell`） |
|---|------|----------|------------------------------|
| 同时多个读者 | ✅ | ❌ | ✅（多个 `&` 容器） |
| 同时写 | ❌ | 独占一个写者 | `borrow_mut` 独占 |
| 校验 | 编译期 | 编译期 | 运行时 |
| LLVM 假设 | 不可变 | noalias | opt-out（`UnsafeCell`） |
| 违反后果 | 编译错误 | 编译错误 | panic |

## 自测

- [ ] 能解释为何图/缓存需要内部可变性
- [ ] 能说出 `Cell` 与 `RefCell` 的核心差别
- [ ] 能说明 `UnsafeCell` 与优化的关系
- [ ] 能判断场景用 `&mut` 还是 `RefCell`/`Mutex`
