# 2.1 · 编译与分发 · 速记

← [05 hub](./05-compilation-dispatch.md) · [章索引](./README.md)

---

## 子节速记

```text
05.1  静态=单态化无vtable · dyn=vtable间接 · inline/LTO/体积控制
05.2  不同 T=不同类型+layout · 同 T 跨文件复用
05.3  object-safe：无泛型方法/无返回 Self 等
05.4  HFT 热路径静态 · enum 静态多态 · cargo bloat
```

## 对照表

| | 静态 | `dyn` |
|---|------|-------|
| 时机 | 编译期 | 运行期 vtable |
| 异构 Vec | enum 等 | `Vec<Box<dyn T>>` |
| HFT | ✅ 优先 | ⚠️ 慎用 |

## 自测

- [ ] 说明 `impl Trait` 是静态还是动态  
- [ ] 为何 `Option<u32>` 与 `Option<u64>` layout 不同  
- [ ] 举两个阻碍 `dyn Trait` 的 trait 方法形态
