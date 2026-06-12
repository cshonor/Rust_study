# Item 17 · 记忆卡片

← [Item 17 目录](./README.md)

| 要点 | 一句 |
|------|------|
| Data race | 安全 Rust 编译期无；unsafe 另论 |
| 死锁 | 编译期不管；设计锁顺序 |
| 首选 | 消息传递 > `Arc<Mutex<T>>` |
| 多结构 | 一个 `State`，一把锁 |
| Guard | 不返回、不持锁调闭包 |
| Send/Sync | 跨线程 move / 共享 `&` 的门槛 |
