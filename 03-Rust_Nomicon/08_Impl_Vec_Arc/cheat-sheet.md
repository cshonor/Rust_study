# 速记 · Vec & Arc · 自测

← [本章目录](./README.md) · [00-overview.md](./00-overview.md)

---

## 三句背诵

1. **MyVec = NonNull + cap + len；push 用 `ptr::write`，pop 用 `ptr::read`，勿对未初始化槽位 Drop/move。**
2. **Drain 初始化时 `len = 0`；RawVec 复用分配逻辑；ZST 不 alloc、`cap = usize::MAX`。**
3. **MyArc = NonNull + PhantomData；Clone Relaxed；Drop Release + Acquire fence；防 forget 溢出 abort。**

## 自测

- [ ] 能画出 MyVec 三字段并说明为何需 `unsafe impl Send/Sync`
- [ ] 能解释 push 为何不能用 `*ptr = x`
- [ ] 能说明 Drain 为何先把 `len` 置 0
- [ ] 能列出 ZST 的四条特殊规则
- [ ] 能解释 Arc Drop 为何 Release 后还要 Acquire fence
- [ ] 能对照 [src/my_vec.rs](./src/my_vec.rs) 与 [src/my_arc.rs](./src/my_arc.rs) 说出关键 API

## 术语表（本章）

| 术语 | 含义 |
|------|------|
| RawVec | 仅 ptr + cap；Vec/Drain 共享分配逻辑 |
| ptr::write/read | 对未初始化/待销毁槽位的正确读写 |
| Release/Acquire | Drop 侧同步其它线程对 `T` 的访问 |
| ZST | size=0；alloc(0) UB，偏移 no-op |

## 源码索引

| 文件 | 演示 |
|------|------|
| [src/my_vec.rs](./src/my_vec.rs) | Vec 主体 |
| [src/raw_vec.rs](./src/raw_vec.rs) | 分配 / ZST |
| [src/into_iter.rs](./src/into_iter.rs) | IntoIter |
| [src/drain.rs](./src/drain.rs) | Drain |
| [src/zst.rs](./src/zst.rs) | ZST 辅助 |
| [src/my_arc.rs](./src/my_arc.rs) | Arc refcount / 屏障 |
