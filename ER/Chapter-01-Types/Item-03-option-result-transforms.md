# Item 3: Prefer Option and Result transforms over explicit match

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：优先使用 Option / Result 的转换方法，而非显式 `match`  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-03-option-result](../ER-demos/item-03-option-result/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `Option`、`match` / `if let` | [6.1 定义枚举](../../Book/06-enums-pattern-matching/6.1-定义枚举.md)、[6.3 if let](../../Book/06-enums-pattern-matching/6.3-if-let.md) |
| `Result`、`?` | [9.2 Result](../../Book/09-error-handling/9.2-Result-与可恢复的错误.md) |
| panic / unwrap | [9.1 panic](../../Book/09-error-handling/9.1-panic-与不可恢复的错误.md)、[9.3 策略](../../Book/09-error-handling/9.3-使用panic还是不用panic.md) |
| 错误类型设计 | [Item 4](./Item-04-idiomatic-error-types.md)（ER） |

---

## 1. 核心知识点与关键定义

| 概念 | 说明 |
|------|------|
| **`Option<T>`** | 值存在 `Some(T)` 或缺失 `None` |
| **`Result<T, E>`** | 成功 `Ok(T)` 或失败 `Err(E)` |
| **转换方法（Transforms / Adaptors）** | `map`、`map_err`、`and_then`、`unwrap_or`、`as_ref` 等，链式处理内部值而少写分支 |
| **`?` 运算符** | 成功则剥出 `Ok`；失败则**提前返回**，并常通过 `From` 做错误类型转换 |
| **解包（Unwrapping）** | `.unwrap()` / `.expect()`：失败即 **`panic!`**（鸵鸟策略，放弃恢复） |

---

## 2. 逻辑脉络与知识点关联

```text
冗长 match
  → if let（只关心一边）
  → map / and_then / map_err 等（函数式链）
  → ?（错误向上传递，最少样板）
```

### 异构错误对齐

- 函数内可能混用多种底层错误（I/O、解析、业务……）。
- **`.map_err(|e| ...)`**：手动把子错误改成对外类型。
- **`?` + `From`**：失败时隐式 `From::from(e)`，子错误「升维」到函数返回的 `E`。

---

## 3. 重点结论与实用要点

1. **少写无意义的 `match`**——只取值或给默认值时用 `if let` / `unwrap_or` / `map` 等，意图更清晰。
2. **库 API：有诊断价值就用 `Result`，别偷换成 `Option`**——「文件不存在」vs「权限拒绝」应留给调用方。
3. **链式转换零成本**——多为 `#[inline]`，优化后与手写 `match` 机器码相当。
4. **在 `&Option<T>` 上操作先 `.as_ref()`**——得到 `Option<&T>`，避免从共享引用里 move 非 `Copy` 值。

---

## 4. 案例与代码要点

### `map_err` + `?` 压缩 I/O 错误

```rust
// 手写 match：每个 Err 都要 return
// match std::fs::File::open("/etc/passwd") { ... }

let f = std::fs::File::open("/etc/passwd")
    .map_err(|e| format!("Failed: {:?}", e))?;
```

### 借用 + `unwrap_or` 的生命周期陷阱

```rust
// ❌ Vec 非 Copy，不能从 &Option<Vec<u8>> 里 move 出来
// encrypt(&self.payload.unwrap_or(vec![]));

// ✅ 在引用层解包
encrypt(self.payload.as_ref().unwrap_or(&vec![]));
```

### 演进对比（示意）

```rust
// 偏冗长
match maybe {
    Some(v) => use_value(v),
    None => default(),
}

// 更紧凑
if let Some(v) = maybe {
    use_value(v);
}
let v = maybe.map(use_value).unwrap_or_default();
```

---

## 5. 易错细节

| 问题 | 说明 |
|------|------|
| **`#[must_use]`** | 忽略 `Result` 返回值会**警告**；故意忽略用 `let _ = ...` 明示 |
| **滥用 `unwrap`** | 等同「这里失败就 panic」；公共库、可恢复场景应避免 |
| **忘记 `as_ref`** | 在 `&Option<T>` 上直接 `map` / `unwrap_or` 可能触发 move 错误 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 03](../ER-拓展索引.md#item-03)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 风格 | 能链式 / `?` 就少 `match` |
| API | 有原因用 `Result`，别全变 `Option` |
| 性能 | 转换方法可内联，不必怕链式 |
| 引用 | `&Option<T>` → 先 `as_ref()` |
| unwrap | 失败即 panic，库代码慎用 |
