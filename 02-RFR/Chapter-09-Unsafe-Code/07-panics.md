# 3.3 Panics（Panic 与不变式）

> 所属：**Great Responsibility** · [← 章索引](./README.md)

← [06 有效性](./06-validity.md) · 下一节 [08 转换](./08-casting.md)

前置 → [06 Validity](./06-validity.md) · [03 MaybeUninit](./03-calling-unsafe-functions.md) · [05 UB 与三大契约](./05-what-can-go-wrong.md)

> ER → [Item 16 panic 安全](../../01-ER/Chapter-03-Concepts/Item-16-avoid-unsafe/README.md)

---

**Panic 本身不是 UB** — 但 panic **栈展开**时若数据结构处于**不一致状态**，`Drop` 可能按错误假设访问内存 → **UB**。

```text
不变式（Invariant）     容器永远应满足的合法约束
不一致窗口             临时打破不变式的一段代码
panic unwind + Drop    在窗口内触发 → 读未初始化 / 双重释放
```

---

## 一、核心问题（原文场景）

手动维护复杂不变式时 — 典型：**先改 `Vec` 的 `len`，再逐个初始化尾部元素** — 若中间调用可能 **panic** 的用户代码：

| 步骤 | 后果 |
|------|------|
| 栈展开（unwind） | 自动对存活变量执行 `Drop` |
| `len` 已增大、尾部未初始化 | `Vec` 析构按 `len` 遍历，对**无效**元素 `drop` |
| 结果 | **二次释放**、**读垃圾内存** → UB |

→ 未初始化元素违反 [06 Validity](./06-validity.md)；典型 [05](./05-what-can-go-wrong.md) 静默 UB 场景。

---

## 二、不变式与不一致窗口

### `Vec<T>` 的不变式（概念）

**`len` == 缓冲区中已完全初始化且 Valid 的元素个数**（`0..len` 每个槽位都是合法 `T`）。

### 不一致窗口

临时打破不变式的代码区间，例如：

```text
1. vec.set_len(len + 1)   ← 不变式破坏，窗口打开
2. 初始化 vec[len-1]      ← 若此处 panic…
3. （窗口应在此关闭）
```

窗口内一旦 panic → `Vec::drop` 仍信 `len` → 对未初始化尾部执行 `drop` → UB。

---

## 三、Panic unwind 致命链（拆解）

1. 手动扩容缓冲区，**`len += 1`**（进入不一致窗口）  
2. 循环构造新元素，某次构造 **`panic!`**  
3. 栈展开 → 调用 **`Vec` 的 `Drop`**  
4. `Drop` 读当前 `len`，对 `0..len` 每个元素 `drop`  
5. 尾部未初始化内存被当作合法 `T` → 读垃圾比特 / 重复释放指针 → **UB**

> **关键**：panic 有定义；**不一致状态下的 `Drop`** 才是 UB。

---

## 四、三类工程对策

### 1. Guard / RAII defer 守卫

守卫在 **`Drop` 时回滚**（正常返回或 panic unwind 都会执行）：

```rust
struct VecLenGuard<'a> {
    vec: &'a mut Vec<u32>,
    old_len: usize,
}

impl Drop for VecLenGuard<'_> {
    fn drop(&mut self) {
        // panic 路径：恢复合法 len
        unsafe { self.vec.set_len(self.old_len) };
    }
}

fn push_with_guard(vec: &mut Vec<u32>, value: u32) {
    let _guard = VecLenGuard {
        old_len: vec.len(),
        vec,
    };
    unsafe {
        vec.set_len(vec.len() + 1);
    }
    // 若下面 panic，guard 回滚 len
    let last = vec.last_mut().unwrap();
    *last = value;
    std::mem::forget(_guard); // 成功：放弃回滚（或改 guard 为「提交」语义）
}
```

> 生产代码更常见：**标准库内部模式** + `forget(guard)` 仅在全部初始化成功后。自定义容器推荐对照 `Vec` 源码。

### 2. `MaybeUninit` 逐元素提交（标准库路线）

**先全部初始化，最后一次性 `set_len`** — 全程无不一致窗口：

```rust
use std::mem::MaybeUninit;

// 概念：在 [MaybeUninit<T>] 里写完所有新元素，再 set_len
// Vec::push / extend 底层近似此模式 → Nomicon 05 · Vec 章
```

| 步骤 | 说明 |
|------|------|
| 1 | 缓冲区用 `MaybeUninit<T>` 承载新槽位 |
| 2 | 在 `MaybeUninit` 上完成构造；中途 panic 不污染已提交 `len` |
| 3 | **全部成功**后一次 `set_len` |

→ [03 MaybeUninit](./03-calling-unsafe-functions.md) · Nomicon [05 Uninit](../../03-Rust_Nomicon/05_Uninit_Mem/README.md)

### 3. 缩小不一致窗口

- **不要**在循环前提前 `set_len`  
- 可能 panic 的逻辑**前置**到修改元数据之前  
- 「打破不变式」与「修复不变式」之间只留**极短、无 panic** 的代码  

---

## 五、错误 vs 修复对比（教学）

### ❌ 错误模式（概念 — 勿模仿）

```rust
// 不一致窗口 + 可 panic 的初始化 = 灾难
// unsafe {
//     vec.set_len(old_len + 1);           // len 已撒谎
//     *vec.get_unchecked_mut(old_len) = f()?; // f() panic → Drop 读未初始化
// }
```

### ✅ 修复思路：`MaybeUninit` + 最后提交

```rust
use std::mem::MaybeUninit;

fn push_uninit_style(slot: &mut MaybeUninit<String>, s: String) {
    slot.write(s); // 只写 MaybeUninit 槽，不提前改 Vec len
}

// Vec 层：所有 slot.write 成功 → 再一次 set_len
// panic 时 len 仍正确，Drop 只处理已初始化前缀
```

用 **Miri** 跑含错误模式的测试可观察 UB 报告 → [12 验证工作](./12-check-your-work.md)

---

## 六、关联与踩坑场景

| 联动 | 说明 |
|------|------|
| [06 Validity](./06-validity.md) | 未初始化元素 = 无效值；`drop` 读无效值 → UB |
| [05 What Can Go Wrong](./05-what-can-go-wrong.md) | 静默损坏、优化后暴露 |
| ER Item 16 | **异常安全（panic safety）**：任何路径都不能脱离合法不变式 |

**常见场景**：手改 `Vec`/`Box` 元数据 · 批量分配构造 · 自定义集合 / arena / 内存池

---

## 七、避坑总结

1. **绝不在**「更新 len/元数据」与「元素初始化完成」之间插入可 panic 代码。  
2. 优先 **`MaybeUninit` + 最后 `set_len`**，消除不一致窗口。  
3. 无法避免临时不一致 → **RAII Guard** panic 回滚。  
4. unsafe 审计须查两条：**Validity** + **panic 异常安全**。

→ 速记：[07-cheat-sheet.md](./07-cheat-sheet.md) · 下一节：[08 转换](./08-casting.md)
