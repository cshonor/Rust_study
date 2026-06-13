# 1.3 Memory Regions（内存区域）

> 所属：**Talking About Memory** · [← 章索引](./README.md)

每个进程有一块独立的**虚拟地址空间**（32 位常见 4GB 用户态视图；64 位更大）。OS 把它切成多段，Rust 的 **栈 / 堆 / 静态** 三分类是语言模型；**Text / Data / BSS / Heap / Stack** 是 OS/链接器视角 — 两者互补。

RFR 日常写 safe Rust 以三分类为主；读 IR、FFI、嵌入式时再对照五分区。

| RFR 三分类 | OS 五分区 |
|------------|-----------|
| 栈 | Stack |
| 堆 | Heap |
| 静态 | Data + BSS（+ 只读 **`.rodata`** 常量，常与 Text 相邻） |

---

## 一、为什么要分区？（整体逻辑）

1. **权限隔离** — 代码段只读，防篡改；数据段可读写；不同页可设不同保护位（NX、RW 等）。
2. **生命周期管理** — 全局变量活全程；局部变量只活一帧；堆上对象寿命由程序/所有权决定 — 分区让 OS 与运行时按不同规则管理。
3. **资源优化** — BSS 不在磁盘存一堆 `0`；栈靠移动栈指针自动回收；堆按需向 OS 要页。

---

## 二、布局示意：两种模型别混

### 核心结论

**现代 Linux / Windows 上跑 Rust：栈和堆是虚拟地址空间里两个（或多块）独立区域，中间隔着库、mmap、guard 等，并不是「同一片连续内存对向生长、直到撞车」。**

教材里常见的「栈 ↓、堆 ↑、中间一块空闲」是**经典简化模型**（裸机 / 早期 OS），用来入门；**≠ PC 上 Rust 程序的真实布局**。

Rust 只负责语言层内存安全（所有权、Drop），**不改变** OS 的栈/堆分区方式。

---

### A. 经典简化模型（入门用，历史/嵌入式）

早期嵌入式、极简系统：整块内存划一片，栈从高地址往下、堆从低地址往上，中间空闲，互相逼近直到溢出。

```text
高地址 ┌─────────────────────────────┐
       │            Stack  ↓         │
       │    （空闲 / 未用区域）        │  ← 经典图：栈堆「对向」逼近
       │            Heap   ↑         │
       ├─────────────────────────────┤
       │              BSS            │
       ├─────────────────────────────┤
       │             Data            │
       ├─────────────────────────────┤
       │        Code / Text          │
低地址 └─────────────────────────────┘
```

**用途**：理解「栈往下长、堆往上长」的**方向直觉**；很多书仍用这个图。**不要把它当成现代桌面 OS 的真实地图。**

---

### B. 现代 Linux / Windows（Rust 程序真实情况）

OS 用**虚拟内存 + 分页**：栈、堆、可执行映射、动态库、`mmap` 文件/匿名映射各占**不同 VA 区间**，由 MMU 映射到物理页；**栈和堆逻辑上分离，不会在同一条「中间缝」里对撞。**

```text
高地址 ┌─────────────────────────────┐
       │  Stack（每线程，固定上限）   │  向下增长；常见几 MB（Linux 默认 ~8MB）
       │  guard 页（栈溢出探测）      │
       ├─────────────────────────────┤
       │  动态库 .so / DLL 映射       │
       │  mmap 区（堆扩展、大分配、    │  malloc/alloc 常经 mmap 拿独立映射
       │   文件映射、匿名映射…）      │
       ├─────────────────────────────┤
       │  Heap（brk 起点 + 上述 mmap）│  空间大，向 OS 按需要页；不与栈相邻
       ├─────────────────────────────┤
       │              BSS            │
       ├─────────────────────────────┤
       │             Data            │
       ├─────────────────────────────┤
       │        Code / Text          │
低地址 └─────────────────────────────┘
```

| | 栈 (Stack) | 堆 (Heap) |
|---|------------|-----------|
| **位置** | 地址空间**偏高**区域（每线程一块） | **中低**区域；与栈**不相邻** |
| **增长** | 高 → 低（压栈） | 经 `brk` / **`mmap`** 向 OS 扩展，**不是**和栈抢同一条缝 |
| **大小** | **固定上限**（线程创建时定） | 受虚拟内存 / ulimit 等限制，通常远大于栈 |
| **溢出** | 递归过深 → **stack overflow**（碰 guard / 越界） | 分配过多 → OOM / 分配失败；**不是因为和栈撞在一起** |

**Rust 对应**：

```rust
fn main() {
    let a: i32 = 10;           // a 在栈（main 帧）
    let b = [0u8; 8];          // 数组也在栈
    let s = Box::new(100);     // Box 句柄在栈；100 在堆（独立映射区）
    let v = String::from("hi"); // 栈上 {ptr,len,cap}；字节在堆
} // 帧弹出；Box/String drop 释放堆
```

栈和堆**物理区块分离**；栈里的 **指针字段**（`Box` / `String` 的 `ptr`）只是**指向**堆上的另一块映射。

```mermaid
flowchart TB
    subgraph modern ["现代 VA（示意，非按比例）"]
        STK["Stack ↓<br/>局部 · 参数 · 返回地址"]
        GUARD["guard / 隔离"]
        LIB["库 · mmap · 大堆块"]
        HEAP["Heap<br/>Box / Vec / String payload"]
        BSS["BSS"]
        DATA["Data"]
        TEXT["Text"]
    end
    STK --> GUARD --> LIB --> HEAP --> BSS --> DATA --> TEXT
```

### 两种模型对照

| | 经典简化模型 | 现代 OS |
|---|-------------|---------|
| 栈堆关系 | 同一片，**对向生长** | **独立区域**，中间有 guard / 库 / mmap |
| 主要风险 | 「中间挤没了」 | 栈：**栈溢出**；堆：**OOM / 分配失败**（各自独立） |
| 何时用 | 建立方向直觉 | 理解真实 Rust 进程、HFT/调优、读 `/proc/maps` |

---

## 三、逐段拆解

### 1. Code / Text（代码段）

| | |
|---|---|
| **存什么** | 编译后的**机器指令**：`main`、你的 `fn`、标准库 `.text` |
| **只读** | 运行时不可改，防 bug / 恶意篡改 |
| **可共享** | 多进程跑同一可执行文件时，物理内存可共享一份代码页 |
| **加载** | 启动时从可执行文件映射进内存 |

### 2. Data（数据段）

| | |
|---|---|
| **存什么** | **已初始化**的全局 / 静态变量（初值非全零或必须写入镜像） |
| **可读写** | 运行时可改（Rust 改 `static mut` 须 `unsafe`） |
| **加载** | 初值保存在可执行文件里，启动时拷入内存 |

Rust 例：`static GLOBAL_INIT: i32 = 100;`

只读常量（如 `'static str` 字面量）常在 **`.rodata`**，权限只读，教材里有时单独提。

### 3. BSS（BSS 段）— 补全

| | |
|---|---|
| **存什么** | **零初始化**的全局 / 静态变量（C 里「未写初值的全局」；加载后等价于全 0） |
| **启动时清 0** | OS/loader 把 BSS 范围一次性置零 → C 里 `int x;` 全局默认为 0 |
| **不占磁盘初值** | 可执行文件只记录 **BSS 大小**，不存成千上万个 `0` → 减小体积 |
| **可读写** | 运行时可改 |

**Rust 与 C 的重要差别**：Rust **`static` 必须写初值**，没有 C 式 `static mut UNINIT: i32;`。

```rust
// ✅ 零初始化 → 链接后通常进 BSS（语义等同「未初始化全局 = 0」）
static mut GLOBAL_BSS: i32 = 0;

// ✅ 非零初值 → Data
static GLOBAL_INIT: i32 = 100;

// ❌ 编译错误：missing initializer
// static mut UNINIT: i32;
```

### 4. Heap（堆）

| | |
|---|---|
| **存什么** | 运行时**动态**申请的块：`Box`、`Vec` 缓冲区、`String` 内部等 |
| **扩展方式** | 向 OS 通过 **`brk` / `mmap`** 要页；现代分配器大块常走 **独立 mmap**，不必紧贴 Data 段 |
| **管理** | C：`malloc`/`free`；Rust：**所有权**在变量 drop 时释放，无 GC |
| **速度** | 比栈慢（分配器、可能向 OS 要页、碎片） |

### 5. Stack（栈）

| | |
|---|---|
| **存什么** | **局部变量**、参数、返回地址、保存的寄存器 — 一整个**栈帧 (frame)** |
| **向下增长** | 新调用压帧 → SP 往低地址；返回弹帧 → SP 回升 |
| **自动管理** | 调用进、返回出，无需手动 free |
| **大小固定** | 每线程栈上限（常见 MB 级）；超限 → 栈溢出 |

---

## 四、栈 vs 堆（对照表）

| | 栈 | 堆 |
|---|-----|-----|
| 分配 / 释放 | 自动（进/出函数） | Rust：`Box::new` 等 + drop |
| 速度 | 极快（改 SP） | 较慢 |
| 大小 | 固定上限 | 受 VM / 分配器 / OS 限制 |
| 典型 Rust | `let x = 42;` | `Box::new(100)` 里的 `100` |
| 与另一者关系 | 现代 OS：**不与堆共享「中间缝」** | 经指针被栈上句柄引用 |

**所有权在做什么**：栈上变量（如 `b: Box<i32>`）**拥有**堆块；`main` 返回、`b` 离开作用域 → **自动 `drop` 堆**，避免泄漏。这是 [04 所有权](./04-ownership.md) 的物理背景。

→ [02 变量深入 · 数据流 vs 槽位](./02-variables-in-depth.md)

---

## 五、Rust 串起来：段 ↔ 所有权 / 借用

| 段 | Rust 关系 |
|----|-----------|
| **Stack** | 局部变量、调用帧；函数返回帧销毁 — 借用检查里的「作用域」多与此对齐 |
| **Heap** | `Box`/`Vec`/`String` 的 owned 数据；栈上往往是**小句柄**（指针+元数据） |
| **Data / BSS** | `static` 活整个进程；`static mut` 需 `unsafe`，多线程还要防数据竞争 → 优先 `OnceLock` 等 |
| **Text** | 只读；与 mut 借用无关 |

**`'static` 生命周期** ≠ 「数据一定在 BSS/Data」：owned 的 `String` 也可 `T: 'static`，人可能在堆上。

→ [08 生命周期](./08-lifetimes.md)

---

## 六、完整示例（合法 Rust）

```rust
// Data：已初始化静态
static GLOBAL_INIT: i32 = 100;

// BSS 语义：零初始化静态（启动时为 0）
static mut GLOBAL_BSS: i32 = 0;

fn main() {
    let a = 42;              // 栈：main 帧
    let b = Box::new(100);   // b 在栈；100 在堆

    unsafe {
        GLOBAL_BSS = 50;     // 改 BSS 区变量（须 unsafe：static mut）
    }
} // b drop → 堆上 100 释放；main 帧弹出
```

| 符号 / 数据 | 大致位置 |
|-------------|----------|
| `main` 机器码 | Text |
| `GLOBAL_INIT` | Data（或 .rodata 若只读常量策略） |
| `GLOBAL_BSS` | BSS（零初始化） |
| `a`、`b`（Box 句柄） | Stack |
| `*b` 的 `100` | Heap |

---

## 七、非常规但仍在虚拟地址空间里的三种映射

除 Text / Data / BSS / Heap / Stack 外，OS 还可把**设备寄存器**、**文件**、**特殊持久内存**映射进同一进程的虚拟地址空间 — 对程序来说都像「读写字节」，底层语义却不同。

```text
  虚拟地址空间（示意，还可包含更多 mmap 区域）
  ┌─────────────────────────────────┐
  │ Stack / Heap / 静态段 …          │  ← 上文五分区
  ├─────────────────────────────────┤
  │ Memory Mapped Files (mmap)      │  ← 文件页缓存
  │ Memory Mapped Registers (MMIO)  │  ← 设备寄存器
  │ NVRAM / 持久内存映射 (pmem 等)   │  ← 断电不丢的特殊 RAM
  └─────────────────────────────────┘
```

| 类型 | 是什么 | 作用 | 典型场景 |
|------|--------|------|----------|
| **Memory Mapped Registers（MMIO）** | 硬件寄存器映射到 VA | 像读写内存一样控设备，少走专用 I/O 指令路径 | 内核/驱动、嵌入式外设；用户态较少直接碰 |
| **Memory Mapped Files** | `mmap` 把文件映到 VA | 像读写内存一样读写文件；OS 管页缓存与写回 | 大文件、DB 引擎、**共享 mmap IPC** |
| **Non-volatile RAM（NVRAM）** | 断电不丢的 RAM（如早期 Optane 类） | 接近内存延迟 + 持久化，少「先写 RAM 再 fsync 落盘」 | 持久化日志、低延迟 KV |

### 1. Memory Mapped Registers（MMIO）

- CPU 访问某段**虚拟地址** → MMU 转到**设备寄存器**，不是普通 DRAM。
- 寄存器值可被硬件异步改变 → 必须用 **`read_volatile` / `write_volatile`**，不能当普通变量做优化。

→ [第 12 章 · 底层内存访问](../Chapter-12-Rust-Without-Standard-Library/06-low-level-memory-access.md) · [第 11 章 FFI / 驱动边界](../Chapter-11-Foreign-Function-Interfaces/README.md)

### 2. Memory Mapped Files（`mmap`）

- Linux：`mmap(2)`；Windows：`MapViewOfFile` 等。
- **优势**：少一次用户态 ↔ 内核缓冲区拷贝；多进程可 **MAP_SHARED** 映射同一文件实现 IPC。
- **注意**：修改不一定立刻落盘（页写回策略）；需 `msync` / `fsync` 等保证持久化语义；**Drop/ munmap** 解除映射 → Book [15.3 Drop](../../00-Book/15-smart-pointers/15.3-使用Drop运行清理代码.md)。

**极简示例**（Linux；Rust 常用 [`memmap2`](https://docs.rs/memmap2) crate）：

```rust
use std::fs::OpenOptions;
use memmap2::MmapMut;

let file = OpenOptions::new().read(true).write(true).open("data.bin")?;
let mut map = unsafe { MmapMut::map_mut(&file)? };

map[0] = map[0].wrapping_add(1); // 像写数组一样写文件映射区

// drop(map) → munmap；脏页由 OS 异步写回磁盘（持久化需 msync/fsync 语义）
```

```c
// 等价 C 心智模型（man 2 mmap）
// void *p = mmap(NULL, len, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
// ((char*)p)[0]++;  munmap(p, len);
```

用户态更常见：**只读映射配置文件**、内存数据库映射数据文件；直接 mmap 设备内存多在内核/嵌入式 HAL。

### 3. Non-volatile RAM（NVRAM）

| | 普通 DRAM | NVRAM / 持久内存 |
|---|-----------|------------------|
| 断电 | 数据丢失 | 数据保留 |
| 程序模型 | 堆/栈数据需刷盘才持久 | 映射区域写入即「可持久」（仍须关注缓存刷新 API） |
| 现状 | 标配 | 专用硬件 + OS `libpmem` 等；多数应用仍用 **文件 + mmap + fsync** |

写数据库 / HFT 持久化时，这是在「堆 vs 栈」之外的第三种**地址空间用法** — 数据既不在栈也不在堆，而在 **文件映射或持久内存映射** 里。

### 和 Rust 的关系（串起来）

| 区域 | Rust 注意点 |
|------|-------------|
| MMIO | **`unsafe` + volatile**；别当 `&mut T` 普通别名优化 |
| mmap 文件 | 映射生命周期、`unsafe map`、**Send/Sync** 与并发映射；Drop 时 unmap |
| NVRAM | 与 `std` 堆不同；持久化语义看 OS/硬件 API，不是所有权 alone |

**一句话**：五分区管「常规程序数据」；这三种管「**通过 MMU 把别的东西也伪装成地址**」— 硬件、文件、持久 RAM。

---

## 延伸阅读

- 布局 / 对齐 → [第 2 章 · Layout](../Chapter-02-Types/02-layout.md)
- IR 里的 `alloca`（栈）与 heap call → [llvm_insight ch04](../../llvm_insight/part02_src_to_machine/chapter04_ir_basic/README.md)
- 共享 `static` → Book [16.3](../../00-Book/16-fearless-concurrency/16.3-共享状态并发.md) · [16.4 Send/Sync](../../00-Book/16-fearless-concurrency/16.4-Send与Sync.md)
- mmap / IPC → Book [14.3.1 Workspace 与微服务 · IPC](../../00-Book/14-cargo-crates/14.3.1-Workspace与微服务.md) · [15.2 Deref · mmap 块](../../00-Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md)
