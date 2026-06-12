# 2. Ownership（所有权）

> [← 章索引](./README.md)

所有权保证：每个值在某一责任域内被**唯一**地负责释放（RAII / `Drop`）。

- **移动 (move)**：所有权转移；旧绑定不再负责 drop（若类型未实现 `Copy`）。
- **复制 (copy)**：按位复制栈上表示；`Copy` 类型不触发 move 语义上的「旧位失效」。

## Drop 顺序

### 局部变量（含参数）

**逆声明顺序** drop（后声明的先 drop）。

**原因**：后声明者可能依赖先声明者（`let a = String::...; let b = &a;`）。若先 drop `a`，`b` 可能悬垂；故先 drop `b` 再 `a`。

### 复合类型内部字段

元组、结构体字段等：**按源码正序** drop。

当前语言设计下单值内部自引用受限，该顺序在现有模型下不破坏安全性。

## 延伸（可另开笔记）

- `ManuallyDrop`、故意泄漏
- `Pin` 与「位置不可移动」
- `async` 状态机里的 drop 与 cancel
- panic 路径上的 unwind 与 drop

Book → [4.1 所有权](../../00-Book/04-ownership/4.1-什么是所有权.md) · ER → [Item 11 RAII](../../01-ER/Chapter-02-Traits/Item-11-drop-raii/README.md)
