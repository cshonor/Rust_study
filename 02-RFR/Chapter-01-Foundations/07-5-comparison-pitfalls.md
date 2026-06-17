# 3.3.5 · 外部可变 vs 内部可变 · 误区与总纲

> 所属：**Borrowing and Lifetimes · 内部可变性** · [← 07 hub](./07-interior-mutability.md)

← [07.4 应用场景](./07-4-use-cases.md) · 下一节 [08 生命周期](./08-lifetimes.md)

---

## 一、完整对比

### 外部 vs `RefCell` 内部可变

| 维度 | 外部（`let` / `let mut`） | `RefCell` 内部可变性 |
|------|---------------------------|----------------------|
| 权限位置 | 外部 `mut` | 容器内 `borrow_flag` |
| 检查 | **编译期** | **运行时** |
| 开销 | 零 | 计数增减（微小） |
| 读写规则 | 编译互斥 | 运行模拟：多读或一写 |
| 外层绑定 | `let` 不可换绑；`let mut` 可 | 外层通常 **`let`** |
| 适用 | 简单路径、极致性能 | `&self`、`Rc`、细粒度改字段 |

### `&mut T` vs 内部可变容器

| | `&mut T` | `RefCell` / `Mutex` |
|---|----------|-------------------|
| 校验 | 编译期 | 运行时 / 锁 |
| 表层 | 独占 `&mut` | 常是 `&Container` |
| 违反 | 编译失败 | panic / 死锁 |

### `Cell` vs `RefCell` vs 外部 `mut`

| | **`let mut` + `&mut`** | **`Cell`** | **`RefCell`** |
|---|------------------------|------------|---------------|
| 检查 | 编译期 | 无借用检查 | 运行时借用 |
| 冲突 | 编译错误 | 无（拷贝） | panic |
| 何时优先 | **默认首选** | 小 Copy + `&self` | 复杂 `T` + `&self` / `Rc` |

---

## 二、常见误区

| 误区 | 纠正 |
|------|------|
| 「有 `&T` 就完全不能改」 | 直接改不行；**内部可变**是显式例外 |
| 「`RefCell` 绕过所有借用规则」 | 规则仍在，**推迟到运行时** |
| 「`RefCell::new` 一定在堆上」 | 单独 `let x = RefCell::new(10)` 常在**栈**；`Rc`/`Box` 才常见堆 |
| 「`let x` 锁死盒内数字」 | 锁的是**绑定**；盒内用 `borrow_mut` |
| 「`Cell` = 小 `RefCell`」 | `Cell` 仅 `Copy`、`get`/`set`；无 `borrow` |
| 「`Mutex` = 多线程 `RefCell`」 | 类似互斥，但还有 **Send/Sync**、中毒、阻塞 |
| Cell/RefCell 可替代一切 `mut` | 能静态检查时优先 **`let mut`** |

---

## 三、一句话总纲

1. **外部可变性**：编译器看 `mut`，冲突**编译期**卡死。  
2. **内部可变性**：外层 `let` 不动，盒内由容器**运行时**管（或 `Cell` 拷贝规避）。  
3. **`RefCell`**：没打破「绑定不可换」，盒内修改权交给容器；违规 **panic**，非 UB。

---

## 四、自测

- [ ] 何时必须用内部可变，而不是 `let mut`？举 2 例  
- [ ] `RefCell` 违规为何是 panic 而不是 UB？  
- [ ] 多线程为何不能用 `RefCell`？  
- [ ] 对照 [07.3](./07-3-cell-vs-refcell.md)：`Cell` 为何不需要借用计数？

→ 下一节：[08 生命周期](./08-lifetimes.md) · [07 速记](./07-cheat-sheet.md)

---

## 对照阅读

- Book → [15.5 RefCell](../../00-Book/15-smart-pointers/15.5-RefCell与内部可变性.md)
- ER → [Item 17 共享状态并行](../../01-ER/Chapter-03-Concepts/Item-17-shared-state-parallelism/README.md)
- 全章 → [05–08 速记](./05-08-borrowing-lifetimes-cheat-sheet.md)
