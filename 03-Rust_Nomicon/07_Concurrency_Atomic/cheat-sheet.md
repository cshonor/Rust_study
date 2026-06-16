# 速记 · Concurrency · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **Safe 杜绝数据竞争（并发写+无同步=UB）；死锁等竞争条件仍可能发生。**
2. **Send = 可跨线程移动；Sync = 可跨线程共享引用（&T: Send ⇔ T: Sync）。**
3. **普通 load/store 不能同步；Relaxed 仅原子性，Release/Acquire 配对，SeqCst 全局序。**

## 自测

- [ ] 能区分数据竞争与一般竞争条件（死锁、TOCTOU）
- [ ] 能解释 `Rc` 为何非 Send/Sync
- [ ] 能描述 TOCTOU + `get_unchecked` 的 UB 路径
- [ ] 能说出 Release/Acquire 与 Relaxed 的选用场景
- [ ] 能对照 [atomics.rs](./src/atomics.rs) 指出各 memory ordering 示例

## 术语表

| 术语 | 含义 |
|------|------|
| 数据竞争 | 并发访问同一内存且至少一方写、无同步 → UB |
| TOCTOU | 检查时有效、使用时已变（time-of-check vs time-of-use） |
| Send / Sync | 编译期线程安全标记 trait |
| happens-before | 跨线程可见性偏序关系 |
| memory ordering | 原子操作的同步强度（Relaxed / Acquire-Release / SeqCst） |
