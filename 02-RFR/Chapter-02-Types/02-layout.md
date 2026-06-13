# 1.2 Layout（布局）

> 所属：**Types in Memory** · [← 章索引](./README.md)

前置 → [01 对齐](./01-alignment.md)（单字段为何要对齐）· 内存分区 → [03.2 OS 布局](../Chapter-01-Foundations/03-2-os-memory-layout.md)

---

## 布局 vs 对齐

| 概念 | 管什么 |
|------|--------|
| **内存对齐 (alignment)** | 每个字段 / 类型的**起始地址**规则（2 的幂）→ [01 对齐](./01-alignment.md) |
| **内存布局 (layout)** | 结构体**字段排列顺序**、**offset**、**padding**、**总 size** — 由 **`repr`** 控制 |

对齐是「每个字段站哪条地址线合法」；布局是「多个字段在 struct 里怎么排队、中间塞多少空字节」。

---

## 对齐在布局里的作用（摘要）

- 字段起始地址必须是**该字段 alignment 要求**的整数倍（通常为 2 的幂；与 `size_of` 相关但不必相等）。
- **32 位**平台常见自然 word **4 字节**；**64 位（x86_64）** 常见 **8 字节** — 见 [01 · x86_64 总结](./01-alignment.md)。
- 未对齐 → 降速或 UB；布局的工作之一就是插入 **padding** 满足对齐。

---

## 三种关键 `repr`

### 对照总表

| `repr` | 字段顺序 | padding | 体积 | 可预测 | 典型用途 |
|--------|----------|---------|------|--------|----------|
| **`Rust`（默认）** | 编译器**可重排** | 自动，偏省空间 | **较小** | ❌ 跨版本不保证 | 日常 Rust |
| **`C`** | **源码书写顺序** | 按 C 规则插入 | 常**更大**（空隙多） | ✅ FFI / 协议 | C 互操作、固定二进制格式 |
| **`packed`** | 顺序固定 | **尽量无** | **最小** | ✅ 但危险 | 网络包、硬件寄存器映射 |

---

### 1. 默认 `repr(Rust)` — Rust 原生布局

- 编译器**可重排字段**：把小字段塞进对齐产生的空隙里。
- **仍满足**每个字段自身的对齐要求 — 不牺牲 CPU 访问合法性。
- **优点**：struct **更小**，padding 更少。
- **缺点**：字段顺序**不固定**、跨编译器版本**不可依赖** → **不能**用于 FFI / 磁盘协议。

```rust
#[derive(Debug)]
struct DefaultLayout {
    a: u8,
    b: u64,
    c: u8,
}
// 编译器可能重排 a/c，减少 padding；具体以 size_of / offset_of 为准
```

---

### 2. `repr(C)` — C 兼容布局

- **严格按源码书写顺序**排列，**不重排**。
- 字段之间按 **C 语言对齐规则** 自动 **padding**。
- **缺点**：容易产生较大 **内部空隙**（struct 总 `size_of` 变大）— 这是 **layout padding**，不是堆 malloc 碎片。
- **优点**：布局**稳定、可预测** → **FFI**、读写字节流 / 文件 / 与 C `struct` 一一对应。

```rust
#[repr(C)]
struct CLayout {
    a: u8,   // offset 0
    // 7 bytes padding（64 位下对齐 u64）
    b: u64,  // offset 8
    c: u8,   // offset 16
    // 7 bytes padding（struct 整体 align 8）
} // size_of 常为 24
```

与 C 头文件对接、生成/bindgen 时**必须**用 `repr(C)`（或更明确的 `repr(C, u8)` 等）。

---

### 3. `repr(packed)` — 紧凑布局

- **强行取消字段间对齐 padding**（或压到 `repr(packed(N))` 指定上限）。
- **极致压缩**体积，字段可能从**非对齐地址**开始。
- **代价**：访问可能 **misaligned** → 慢；部分 CPU **fault**；Rust 里可能 **UB**。
- **仅用于**：必须逐字节匹配的外部二进制格式；读完后尽快拷贝到对齐的 Rust 类型。

```rust
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32, // 可能从 offset 1 开始 — 未对齐
}
// 读 b 常需 copy 到局部变量或 unsafe 解引用
```

→ unsafe / validity → [第 9 章](../Chapter-09-Unsafe-Code/06-validity.md)

---

## 同一字段集：三种 repr 对比（观测）

```rust
use std::mem::{align_of, offset_of, size_of};

macro_rules! show_layout {
    ($name:ty) => {
        println!(
            "{}: size={}, align={}, a@{}, b@{}, c@{}",
            stringify!($name),
            size_of::<$name>(),
            align_of::<$name>(),
            offset_of!($name, a),
            offset_of!($name, b),
            offset_of!($name, c),
        );
    };
}

#[derive(Debug)]
struct RustLayout { a: u8, b: u32, c: u8 }

#[repr(C)]
struct CLayout { a: u8, b: u32, c: u8 }

#[repr(packed)]
struct PackedLayout { a: u8, b: u32, c: u8 }

fn main() {
    show_layout!(RustLayout);
    show_layout!(CLayout);
    show_layout!(PackedLayout);
}
```

**典型现象（x86_64，以你机器打印为准）**：

| 类型 | size 趋势 | 说明 |
|------|-----------|------|
| `RustLayout` | 常 **8** | 重排后压缩 |
| `CLayout` | 常 **12** | `a` + 3 pad + `b` + `c` + 3 pad |
| `PackedLayout` | 常 **6** | 无 pad；`b`/`c` 可能 misaligned |

---

## 观测工具

| API | 作用 |
|-----|------|
| **`size_of::<T>()`** | 类型**总占用**字节（含尾部 padding） |
| **`align_of::<T>()`** | 类型整体 **alignment** 要求 |
| **`offset_of!(Type, field)`** | 字段相对 struct 起始的**字节偏移** |

```rust
use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Example { a: u8, b: u64 }

assert_eq!(offset_of!(Example, a), 0);
assert_eq!(offset_of!(Example, b), 8);
assert_eq!(size_of::<Example>(), 16);
assert_eq!(align_of::<Example>(), 8);
```

- 堆分配 layout → `std::alloc::Layout`（size + align 一起交给分配器）
- IR 里的 `align` → [llvm_insight ch05](../../llvm_insight/part02_src_to_machine/chapter05_ir_advanced_type/README.md)

---

## 选型一句话

| 需求 | 选 |
|------|-----|
| 普通 Rust 结构体 | 默认 **`repr(Rust)`** |
| 给 C / 固定二进制用 | **`repr(C)`** |
| 逐字节贴协议、能承担 unsafe | **`repr(packed)`** + 极度谨慎 |

---

## 易错点

| 易错 | 纠正 |
|------|------|
| 「布局 = 对齐」 | **对齐**是规则；**布局**是字段顺序 + padding 的具体排法 |
| 「`repr(C)` 更省内存」 | 通常 **更占字节**（不重排、空隙多） |
| 「默认 struct 字段顺序 = 源码顺序」 | **只有 `repr(C)` / `packed`** 保证顺序 |
| 「`packed` 只是体积小」 | 可能 **UB / crash**，不是免费午餐 |
| 「padding = 堆碎片」 | struct **内部**填充；随对象一起释放 |

---

## 延伸

- 枚举 niche optimization → [03 复合类型](./03-complex-types.md)
- 自定义布局 / unsafe → [第 9 章 Unsafe](../Chapter-09-Unsafe-Code/README.md)
- Book → [19.3 高级类型 · 内存布局](../../00-Book/19-advanced-features/19.3-高级类型.md)
