# 5.3 方法语法 demo

笔记：[5.3-方法语法.md](../5.3-方法语法.md)

## 运行

```bash
cargo run
```

## 三种 `self`（main 第一节）

| 签名 | 示例 | 调用后实例 |
|------|------|------------|
| `&self` | `area()` | 仍可用 |
| `&mut self` | `set_size()` | 仍拥有，字段已改 |
| `self` | `destroy_to_tuple()` | **move，不可用** |
| 无 self | `Rectangle::new()` | 关联函数 |

## 自动引用

- `r.area()` → `(&r).area()`（self 自动加 `&`）
- `r1.can_hold(&r2)`：**other 须手写 `&`**

## 多 impl

同一 `Rectangle` 可拆多块 impl（只读 / 修改 / 关联函数），见 `main.rs`。
