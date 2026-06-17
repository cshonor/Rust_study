# 3.4 · Casting · 速记

← [08 Casting](./08-casting.md) · [Ch02 Layout](../Chapter-02-Types/02-layout.md) · [章索引](./README.md)

---

## repr

| | `repr(Rust)` | `repr(C)` |
|---|--------------|-----------|
| 字段顺序 | 不保证 | 声明顺序 |
| transmute | 跨泛型 ❌ | 协议/FFI 可用（仍 unsafe） |
| 用途 | 日常 Rust | C / 二进制 / 硬件 |

## 指针 cast 两道门槛

1. **对齐** — `*u8` → `*u64` 须地址对齐  
2. **Provenance** — `ptr as usize` 丢出处；`usize as ptr` 解引用通常 UB  

## `as` vs `transmute`

| | `as` | `transmute` |
|---|------|-------------|
| 对象 | 指针类型 | 任意同尺寸位模式 |
| 保留 | provenance（规则内） | 无布局语义 |

## 决策

Rust 内部 → 默认 repr，不跨泛型 transmute  
协议/FFI → `repr(C)`  
指针 → 对齐 + 忌 usize 往返

## 自测

- [ ] 为何 `Foo<A>` 与 `Foo<B>` 不能transmute？  
- [ ] `repr(packed)` 主要风险是什么？  
- [ ] provenance 与「指针只是地址」差在哪？
