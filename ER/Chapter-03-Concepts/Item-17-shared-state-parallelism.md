# Item 17: Be wary of shared-state parallelism

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：对共享状态的并行保持警惕  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| 线程、`spawn` | [16.1 使用线程](../../Book/16-fearless-concurrency/16.1-使用线程同时运行代码.md) |
| 消息传递（优先） | [16.2 消息传递与通道](../../Book/16-fearless-concurrency/16.2-消息传递与通道.md) |
| `Mutex` / `Arc<Mutex<T>>` | [16.3 共享状态并发](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| `Send` / `Sync` | [16.4 Send 与 Sync](../../Book/16-fearless-concurrency/16.4-Send与Sync.md) |
| 内部可变性 | [15.5 RefCell](../../Book/15-smart-pointers/15.5-RefCell与内部可变性.md) |
| 借用与 data race | [Item 15](./Item-15-borrow-checker.md) |

---

## 1. 核心知识点与关键定义

### 共享状态并行（Shared-state parallelism）

- 多线程通过**共享内存**通信。
- 经典风险：**数据竞争 (data races)**、**死锁 (deadlocks)**。

### 数据竞争三要素

1. 两线程访问**同一内存**
2. 至少一方**写入**
3. **无同步**规定顺序

→ 满足则 data race（C/C++ 中 UB；Rust **安全代码编译期杜绝**）。

### 死锁

- 线程互相等锁 → 永久停滞。
- **Rust 编译期无法防止死锁**。

### `Mutex<T>`

- 包装共享数据；`lock()` → **`MutexGuard`**（RAII，`Deref`/`DerefMut`，`Drop` 解锁）。

### 标记 trait

| Trait | 含义 |
|-------|------|
| **`Send`** | 所有权可**跨线程转移** |
| **`Sync`** | `&T` 可**跨线程共享** |

---

## 2. 逻辑脉络

```text
借用规则（多个 & 或 一个 &mut）≈ 消除 data race 的逻辑前提
         ↓
安全 Rust：编译期无 data race
         ↓
共享可变状态：Arc（多线程引用计数）+ Mutex（内部可变）→ Arc<Mutex<T>>
         ↓
多 Mutex / 锁顺序不一致 → 死锁（编译期不管）
         ↓
对策：优先消息传递；合并 State；极小锁域
```

### 锁倒置（Lock inversion）

- 线程 A：`锁1 → 锁2`
- 线程 B：`锁2 → 锁1`
- → 经典死锁。

---

## 3. 重点结论与实用要点

### 优先：用通信代替共享内存

- Go 格言在 Rust 中对应 **[16.2 `mpsc`](../../Book/16-fearless-concurrency/16.2-消息传递与通道.md)** 等通道。
- 共享内存是**不得已**时的选项。

### 关联数据：一个 `Mutex` 包一整块 `State`

- ❌ 多个独立 `Mutex` 保护逻辑上必须一致的结构 → 同步缝隙、死锁、状态撕裂。
- ✅ 合并为单一 `State`，**一把锁**保护全部相关字段。

### 锁作用域极简原则

| 规则 | 原因 |
|------|------|
| `{}` 尽早放锁 | 缩短持锁时间 |
| **勿把 `MutexGuard` 返回给调用方** | 锁生命周期失控 → 死锁 |
| **持锁时勿调外部闭包** | 未来逻辑可能再抢锁 |

---

## 4. 案例与代码要点

### 编译期拦截「双 `&mut`」

```rust
// 两个 spawn 各要 &mut account → ❌
// cannot borrow `account` as mutable more than once at a time
```

借用检查在**未加 Mutex 前**就挡住 C++ 式并发漏洞。

### 锁倒置：`players` + `games` 两把锁

| 方法 | 顺序 |
|------|------|
| `add_and_join` | `players` → `games` |
| `ban_player` | `games` → `players` |

→ 并发死锁。

### 错误「修复」：拆锁域仍死锁 → 状态撕裂

- 用大括号让两方法**不同时持两把锁**，可避免交叉死锁。
- 但中间窗口：A 写入 `players` 后释放，B 完成封禁，A 再锁 `games` 把已封禁用户加进游戏 → **逻辑不一致**。
- 正解仍是 **单一 `State` + 统一锁顺序**，而非拆缝。

---

## 5. 易错细节

| 细节 | 说明 |
|------|------|
| **锁中毒 (poisoning)** | 持锁线程 panic → 数据可能半更新；`lock().unwrap()` 传播 panic 是常见务实选择 |
| **`Rc` / `RefCell` 跨线程** | 非 `Send`/`Sync` → 编译报错；须 `Arc` + `Mutex` |
| **缩小锁域过度** | 为避死锁拆锁 → 可能引入**撕裂**，比死锁更难查 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 17](../ER-拓展索引.md#item-17)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| Data race | 安全 Rust 编译期无；unsafe 另论 |
| 死锁 | 编译期不管；设计锁顺序 |
| 首选 | 消息传递 > `Arc<Mutex<T>>` |
| 多结构 | 一个 `State`，一把锁 |
| Guard | 不返回、不持锁调闭包 |
| Send/Sync | 跨线程 move / 共享 `&` 的门槛 |
