# as / into / get / try · 速记

← [01-2 详例](./01-2-as-into-get-try-examples.md) · [01 命名](./01-naming-practices.md)

---

## 口诀

| 前缀 | 所有权 | 失败 |
|------|--------|------|
| `as_` | 借，不消耗 | — |
| `into_` | 消耗 self | — |
| `get_` | 借 | `None` |
| `try_` | 通常保留 | `Err` |

## 代表 API

`as_str` / `as_ref` · `into_iter` / `into_inner` · `get` / `get_mut` · `try_lock` / `try_into`

## 对比

`v[5]` panic · `v.get(5)` → `None` · `lock()` 阻塞 · `try_lock()` → `Err`

## 自测

- [ ] `s.as_str()` 两次后 `s` 还能打印吗？  
- [ ] `into_iter` 后还能 `println!("{:?}", v)` 吗？  
- [ ] 锁被持有时 `try_lock()` 会怎样？
