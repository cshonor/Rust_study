# Item 18: Don't panic

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：不要恐慌 / 避免使用 `panic!`  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-18-dont-panic](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `panic!` 机制 | [9.1 panic 与不可恢复的错误](../../Book/09-error-handling/9.1-panic-与不可恢复的错误.md) |
| `Result` 与 `?` | [9.2 Result 与可恢复的错误](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| 何时 panic | [9.3 使用 panic 还是不用 panic](../../Book/09-error-handling/9.3-使用panic还是不用panic.md) |
| 错误传播模式 | [Item 3](../Chapter-01-Types/Item-03-option-result-transforms/README.md) |
| FFI / panic 边界 | [Item 16](../Item-16-avoid-unsafe/README.md)、[Item 34](../Chapter-06-Beyond-Standard-Rust/Item-34-ffi-boundaries/README.md) |
| 锁中毒 | [Item 17](../Item-17-shared-state-parallelism/README.md) |
| `no_panic` + CI | [Item 32](../Chapter-05-Tooling/Item-32-ci/README.md)（待补） |

---

## 1. 核心知识点与关键定义

### `panic!` 的定位

- 面向**不可恢复的 bug**（契约/不变量被破坏）。
- 默认：**终止当前线程**（可配置为 abort 整个进程）。

### `catch_unwind` 的局限

- 标准库提供 `std::panic::catch_unwind`，**不是** Rust 版的 `try-catch`。
- 用其「恢复业务错误」是**反模式**（见 §2）。

### Abort vs Unwind

| 模式 | 行为 |
|------|------|
| **unwind**（默认多数 target） | 沿栈展开，跑 `Drop` |
| **abort** | `Cargo.toml` `panic = "abort"` 或部分 target（如 WASM）→ **直接终止**，`catch_unwind` 无效 |

### 异常安全性（Exception safety）

- panic 发生在**数据结构更新中途** → 可能处于**不一致**状态。
- 在 panic 可能发生的上下文里维持不变量**极难** → 别指望 `catch_unwind` 当通用恢复手段。

---

## 2. 逻辑脉络

```text
C++/Java 思维：catch_unwind ≈ try-catch
         ↓
阻碍 1：panic=abort → catch 根本跑不到
阻碍 2：状态撕裂 → exception safety 不成立
阻碍 3：panic 越过 FFI → UB
         ↓
库代码正路：Result + ? → 把决策交给调用者（passing the buck）
```

---

## 3. 重点结论与实用要点

### 最高准则：优先 `Result`，不是 `panic!`

- 可预期、可恢复的错误 → **`Result<T, E>`** + **`?`**。

### 允许 `panic!` 的场景

| 场景 | 说明 |
|------|------|
| **`main` / 二进制顶层** | 无更上层可推诿 |
| **内部状态损坏** | 非外部非法输入，而是 invariant 已破 |
| **成对 API** | fallible + infallible 并存，如 `from_utf8` / `from_utf8_unchecked` |

### 公共 API 文档

- 可能 panic 的函数 → **`# Panics`** 段写清触发条件。

---

## 4. 案例与代码要点

### 反模式：`catch_unwind` 掩盖除零

```rust
fn divide_recover(a: i64, b: i64, default: i64) -> i64 {
    let result = std::panic::catch_unwind(|| divide(a, b));
    match result {
        Ok(x) => x,
        Err(_) => default, // panic=abort 时进程直接死，走不到这里
    }
}
```

正解：`divide` 返回 `Result<i64, DivideError>`，由调用者决定默认值或向上传播。

### 隐性 panic 来源（不写 `panic!` 也会 panic）

| 来源 | 例子 |
|------|------|
| 显式 | `panic!`、`unreachable!` |
| `Option`/`Result` | `.unwrap()`、`.expect()`、`.unwrap_err()` |
| 边界检查 | `slice[i]` 越界 |
| 算术 | 调试 build 下 `x / 0`（release 行为见语言/优化） |

→ 不能靠 Code Review 盯；用类型、`Result`、测试、**`no_panic`**（§6）交给机器。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **`catch_unwind` 当异常** | abort 模式、FFI、状态撕裂三重失效 |
| **库内 `unwrap`** | 把可恢复错误变成线程/进程崩溃 |
| **FFI 边界** | Rust panic **不得**传播到 C 等调用方 → UB；须 `catch_unwind` + 转错误码或 abort 策略（见 Item 34） |
| **持锁 panic** | [锁中毒](../Item-17-shared-state-parallelism/README.md) — 另一路「panic 的代价」 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 18](../ER-拓展索引.md#item-18)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 默认 | **`Result` > `panic!`** |
| panic 用途 | 不可恢复 bug / 顶层 `main` |
| catch_unwind | 不是 try-catch；别用来「修」业务错误 |
| abort | 配置后 catch 无效 |
| 文档 | 公共 API 写 `# Panics` |
| 隐性 | unwrap、越界、除零 — 用机器查 |
