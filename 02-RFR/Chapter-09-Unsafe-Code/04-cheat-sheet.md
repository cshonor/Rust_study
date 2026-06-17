# 2.3 · unsafe trait · 速记

← [04 unsafe trait](./04-unsafe-traits.md) · [01 五类超能力](./01-unsafe-keyword.md) · [Ch02 §09.3 Send/Sync](../Chapter-02-Types/09-3-send-sync-unpin.md)

---

## 判定

错误 impl → **安全代码 UB** → 用 `unsafe trait` + `unsafe impl`。

## 语法

```rust
unsafe trait DangerousMarker {}
unsafe impl DangerousMarker for MyType {}
```

## Send / Sync

- **unsafe auto trait**；空 marker  
- 错 impl → 数据竞争 UB  
- 字段均满足 → 自动推导；否则手动 `unsafe impl` + 审计  

## Unpin ≠ unsafe trait

- `trait Unpin` — 普通 `impl`  
- 风险在 `Pin::new_unchecked`（unsafe fn）

## 对照

| | unsafe trait | unsafe fn |
|---|--------------|-----------|
| 谁担责 | **实现者** | **调用者** |
| 例子 | `Send`/`Sync` | `assume_init` |

## 自测

- [ ] 为何 `Send` 是 unsafe trait 而不是普通 trait？  
- [ ] `Unpin` 为何不需要 `unsafe impl`？  
- [ ] 五大 unsafe 超能力里哪一条对应 `unsafe impl Send`？
