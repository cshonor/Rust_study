# 1.4 · DST 与宽指针 · 速记

← [04 hub](./04-dst-wide-pointers.md) · [章索引](./README.md)

---

## 子节速记

```text
04.1  DST 不能按值 · [T]/str/dyn Trait
04.2  瘦 8B · 宽 16B · len vs vtable
04.3  vtable 含 drop/size/方法 · 间接调用
04.4  Vec=ptr+len+cap · FFI 传 ptr+len
```

## 对照

| 宽指针 | 第二字 |
|--------|--------|
| `&[T]` / `&str` | len |
| `&dyn Trait` | vtable |

## 自测

- [ ] `size_of::<&[u32]>()` 在 64 位是多少？  
- [ ] 为何 `dyn Iterator` 须写 `Item = T`？  
- [ ] FFI 为何不能传 `&dyn Trait`？
