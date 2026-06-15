# Rustonomicon 读书笔记

> 配套 **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)** — *The Dark Arts of Unsafe Rust*  
> 源码示例：[01_Safe_Unsafe/](./01_Safe_Unsafe/) … [11_NoStd/](./11_NoStd/)

编译环境：`rustup run nightly`（按需）· Edition **2018**

---

## 01 Safe and Unsafe（[01_Safe_Unsafe/](./01_Safe_Unsafe/)）

官方标题 **Meet Safe and Unsafe**。正文无「第一章」编号，但这是全书第一个核心主题板块，为后续 unsafe 编程建立心智模型。

### 1. Safe Rust 与 Unsafe Rust 的双重世界

Rust 可视为两种语言的结合：

| | Safe Rust | Unsafe Rust |
|---|-----------|-------------|
| 定位 | 真正的 Rust 语言 | 与 Safe 语义相同，但开放额外能力 |
| 保证 | 类型安全、内存安全；无悬垂指针、use-after-free、UB | 无上述保证——误用即 UB |
| 用途 | 日常开发 | 系统级底层控制（类似 C） |

### 2. `unsafe` 关键字的两种用法

`unsafe` 是安全与非安全世界的**边界**：

1. **声明契约（caller/implementor 责任）**  
   在函数或 trait **声明**上加 `unsafe`，表示调用者或实现者必须阅读文档并手动保证前提条件成立。  
   例：`unsafe fn`、`unsafe trait Send`。

2. **履行契约（程序员已验证）**  
   在代码块或 trait **实现**上加 `unsafe`，表示程序员已确认此处所有 unsafe 操作合法、符合契约。  
   例：`unsafe { ... }`、`unsafe impl Send for MyType`。

### 3. Unsafe Rust 的 5 种额外能力

仅此五项；**任意一项误用 → 未定义行为 (UB)**，编译器可对程序做任意破坏：

1. 解引用**原生指针**（raw pointers）
2. 调用 **`unsafe` 函数**（含 FFI、编译器 intrinsics）
3. 实现 **`unsafe` trait**（如 `Send`、`Sync`）
4. 访问或修改**可变静态变量**（`static mut`）
5. 访问 **`union` 字段**

### 4. 信任不对称原则

安全与非安全之间的信任关系是**不对称**的：

- **Safe → Unsafe**：Safe Rust 必须**无条件信任** Unsafe 代码被正确编写（标准库大量 API 内部依赖 unsafe）。
- **Unsafe → Safe**：Unsafe 代码**不能**假设用户提供的 Safe 代码一定正确。

例：`BTreeMap` 内部用 unsafe，若用户自定义 `Ord` 实现有逻辑 bug，unsafe 层必须足够健壮，**不能**因此产生内存 UB——最多 panic 或返回错误结果，不能破坏内存安全 invariant。

### 5. 安全性的非局部性（Non-locality）

**本章最重要洞察**：在含 unsafe 的程序中，修改一段看似无害的 Safe 代码，也可能让整个库变得 **unsound** 并触发 UB。

原因：unsafe 操作的正确性往往依赖程序其他处的「安全」状态维护。例：`Vec` 的 unsafe 内部逻辑依赖 `len`、`cap` 字段始终一致；若模块外 Safe 代码能直接篡改这些字段，invariant 即被破坏。

**对策**：通常唯一可靠的做法是利用**模块可见性（privacy）**，在模块边界建立屏障，把 unsafe 细节封装成对外完全 Safe 的抽象 API。

---

## 02 Data Layout（[02_Data_Layout/](./02_Data_Layout/)）

## 03 Lifetime and Variance（[03_Lifetime_Variance/](./03_Lifetime_Variance/)）

## 04 Type Casting（[04_Type_Cast/](./04_Type_Cast/)）

## 05 Uninitialized Memory（[05_Uninit_Mem/](./05_Uninit_Mem/)）

## 06 Ownership and RAII / OBRM（[06_OBRM_RAII/](./06_OBRM_RAII/)）

## 07 Panic Safety（[07_Panic_Safety/](./07_Panic_Safety/)）

## 08 Concurrency and Atomics（[08_Concurrency_Atomic/](./08_Concurrency_Atomic/)）

## 09 Implementing Vec and Arc（[09_Impl_Vec_Arc/](./09_Impl_Vec_Arc/)）

## 10 FFI（[10_FFI/](./10_FFI/)）

## 11 No Standard Library（[11_NoStd/](./11_NoStd/)）
