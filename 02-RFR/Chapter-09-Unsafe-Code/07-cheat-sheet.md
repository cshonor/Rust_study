# 3.3 · Panic 与不变式 · 速记

← [07 Panics](./07-panics.md) · [06 Validity](./06-validity.md) · [章索引](./README.md)

---

## 关键区分

| | Panic | UB |
|---|-------|-----|
| 定义 | 有（栈展开或 abort） | 无保证 |
| 本节的 UB | — | **不一致状态下 Drop** |

## 不一致窗口

`set_len` 增大 **先于** 尾部初始化 → 中间 panic → `Vec::drop` 读未初始化 → UB

## 三对策

1. **Guard** — panic 时 `set_len(old_len)` 回滚；成功则提交 `old_len`  
2. **MaybeUninit** — `spare_capacity_mut` + `write`，最后 `set_len`（`Vec::push` 路线）  
3. **缩小窗口** — 元数据修改与初始化之间无 panic  

## 可运行对比

- ❌ `set_len` → `build()`（可 panic）  
- ✅ `MaybeUninit::write` → `set_len`  

## 审计两条

- Validity（06）  
- panic safety（本节）

## 自测

- [ ] 为何 panic 本身不是 UB，但本节场景会产生 UB？  
- [ ] `Vec::push` 为何不先 `len+=1` 再构造元素？  
- [ ] Guard「提交」时为何改 `old_len` 即可取消回滚？
