# Item 34: Control what crosses FFI boundaries

> **Effective Rust** · [Chapter 6 — Beyond Standard Rust](../ER-本书目录.md)  
> **中文**：控制跨越 FFI 边界的内容  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-34-ffi-box](../ER-demos/item-34-ffi-box/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `unsafe`、裸指针 | [19.1 不安全 Rust](../../Book/19-advanced-features/19.1-不安全Rust.md) |
| 所有权 | [4.1 什么是所有权](../../Book/04-ownership/4.1-什么是所有权.md) |
| `Drop` / RAII | [Item 11](../Chapter-02-Traits/Item-11-drop-raii.md) |
| 生命周期、引用 | [Item 14](../Chapter-03-Concepts/Item-14-lifetimes.md) |
| 避免手写 unsafe | [Item 16](../Chapter-03-Concepts/Item-16-avoid-unsafe.md) |
| Panic 与 FFI | [Item 18](../Chapter-03-Concepts/Item-18-dont-panic.md) |
| FFI 多版本 ODR | [Item 25](../Chapter-04-Dependencies/Item-25-dependency-graph.md) |
| 自动生成绑定 | [Item 35](./Item-35-bindgen.md)（待补） |

---

## 1. 核心知识点与关键定义

### FFI（Foreign Function Interface）

- Rust 与其他语言互操作；常以 **C ABI** 为「最小公约数」（无 GC、无异常、无模板）。

### 不安全边界

- 跨 FFI = 离开 Rust 内存安全保证 → **所有 FFI 调用 `unsafe`**（Item 16 原则在此**必须打破**，但须封装）。

### 名称修饰

| 机制 | 作用 |
|------|------|
| **`extern "C"`** | C ABI；隐式 **`#[no_mangle]`** → 符号名不被 Rust 编码 |
| C++ / Rust 默认 | name mangling → 与 C 全局命名空间不兼容 |

### C 兼容布局

```rust
#[repr(C)]
pub struct FfiPoint {
    pub x: u32,
    pub y: u32,
}
```

- 禁止 Rust 默认字段重排 → 与 C struct **布局、对齐一致**。

---

## 2. 逻辑脉络

```text
Rust 侧：借用 + 生命周期（Item 14/15）
         ↓ 过 FFI
C 侧：裸指针，无借期、可 UAF、可 data race
         ↓
重建安全：unsafe 在内，safe wrapper 对外（Item 16）
         ↓
所有权跨界：同侧 alloc/free；Box ↔ 裸指针（Item 11 Drop）
         ↓
panic 不得越过 FFI（Item 18 → catch_unwind / abort）
```

---

## 3. 重点结论与实用要点

### 同侧分配、同侧释放

- C `malloc` → C `free`；Rust `Box` → Rust `drop`。**禁止混用**。

### 明确宽度整数

- 优先 **`u32` / `i64`** 等固定宽度。
- 遗留 `int` → `std::os::raw::c_int` 等；**别假设**两边 `int` 同宽。

### Safe wrapper 模式

```text
struct Wrapper { ptr: *mut c_void }  // 内部裸指针
impl Wrapper {
    pub fn new(...) -> Result<Self, Error> { ... }  // 对外 safe
}
impl Drop for Wrapper { /* C free 或 Box drop */ }
```

- 不让 `extern "C"` 调用散落全库。

### Panic 不得跨 FFI

- 未捕获 panic 展开到 C → **UB**。
- 对外 `extern "C"` 入口：`catch_unwind` + 错误码，或 **`panic = "abort"`** 策略（Item 18）。

---

## 4. 案例与代码要点

### 字符串：`String` vs C string

| Rust | C |
|------|---|
| UTF-8 + length，可含 `\0` | `\0` 结尾 `char*` |

- 传出：**`CString::new(s)?`** → `into_raw()` 给 C。
- 借入：**`CStr::from_ptr`**（unsafe，须保证指针有效、UTF-8 或按字节处理）。

### `Box` ↔ C 所有权

```rust
// ❌ 返回 &raw from Box 局部变量 → 函数结束 drop → UAF

// Rust → C：交出所有权
let ptr = Box::into_raw(b);

// C → Rust：收回并释放
let b = unsafe { Box::from_raw(ptr) };
// drop(b) 释放堆
```

### 勿给 C 指针假生命周期

```rust
// ❌ C 临时指针 / 已 free
let s: &FfiStruct = unsafe { &*p };
```

- 裸指针**没有** `'a`；别假装有借用检查保护。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **手写 `extern` 签名错** | `extern "C"` 无类型检查链接 → **运行时才崩**；用 **bindgen**（Item 35） |
| **签名略有不一致** | 链接器仍通过 → 栈/layout 错乱 |
| **混用 allocator** | 跨语言 free = 经典 crash |
| **FFI crate 多版本** | C 符号 ODR 冲突（Item 25） |
| **panic 泄漏** | UB |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 34](../ER-拓展索引.md#item-34)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| ABI | **`extern "C"`** + **`#[repr(C)]`** |
| 内存 | **谁分配谁释放** |
| 整数 | 固定宽度；慎 `c_int` |
| 封装 | unsafe 在内，**safe API** 在外 |
| Box | **`into_raw` / `from_raw`** |
| 字符串 | **`CString` / `CStr`** |
| panic | **不过 FFI** |
| 绑定 | 手写危险 → **bindgen** |
