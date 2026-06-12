# Item 33 · 记忆卡片

← [Item 33 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 栈 | **core**（无堆）→ **alloc**（堆）→ **std**（OS） |
| 声明 | `#![cfg_attr(not(feature="std"), no_std)]` |
| Feature | **`std` 加法**，别 `no_std` 减法 |
| 守卫 | **CI 交叉编译** bare-metal |
| OOM | **`try_reserve`** / `try_new` |
| 缺啥 | HashMap→BTree；Mutex→spin |
