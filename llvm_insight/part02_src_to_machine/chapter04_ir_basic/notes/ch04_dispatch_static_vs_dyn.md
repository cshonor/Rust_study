# ch04 — 静态分发 vs 动态分发（IR 对照）

> 源码：`llvm_insight/src/lib.rs` · IR 归档：`ir_samples/optimize_compare/ch04_dispatch_O0.ll`（及 `O3`）  
> RFR 理论 → [05 编译与分发](../../../../02-RFR/Chapter-02-Types/05-compilation-dispatch.md)

## 复现

```bash
cargo rustc --manifest-path llvm_insight/Cargo.toml -p llvm_insight_lab -- -C opt-level=0 --emit=llvm-ir
# .ll 在 target/debug/deps/；或直接使用 ir_samples 已归档副本
```

实验函数（均 `#[inline(never)]` 便于在 IR 里看到独立 `define`）：

| Rust | 分发 |
|------|------|
| `process_static<T: TickHandler>(h: &T)` | **单态化** — 每个 `T` 一份 `define` |
| `process_dyn(h: &dyn TickHandler)` | **vtable** — 间接 `call` |

## IR 要点（O0，`ch04_dispatch_O0.ll`）

### 1. 两份 `process_static`（monomorph）

```llvm
; process_static<HandlerA>
define i64 @...process_static...hccf3183...(ptr align 8 %h) {
  %_0 = call i64 @...HandlerA...on_tick...(ptr align 8 %h)
  ret i64 %_0
}

; process_static<HandlerB>  — 另一 symbol，直接 call B::on_tick
define i64 @...process_static...h9130f767...(ptr align 8 %h) { ... }
```

**无 vtable**；调用目标在编译期写死在 `call` 里。

### 2. 一份 `process_dyn` + 全局 vtable 常量

胖指针拆成两个参数：`%h.0` = data ptr，`%h.1` = vtable ptr。

```llvm
@vtable.0 = private constant <{ [24 x i8], ptr }> <{
  [24 x i8] c"...",   ; drop / size / align 等元数据（24B）
  ptr @...HandlerA...on_tick...
}>

define i64 @...process_dyn...(ptr align 1 %h.0, ptr align 8 %h.1) {
  %1 = getelementptr inbounds i8, ptr %h.1, i64 24
  %2 = load ptr, ptr %1                    ; 从 vtable 取 on_tick 地址
  %_0 = call i64 %2(ptr align 1 %h.0)      ; 间接 call
  ret i64 %_0
}
```

对应 Rust 笔记里的：**先读 vtable → 再间接跳转**（offset 24 处为首个 trait 方法槽，与 rustc 版本/ trait 有关，以实际 `.ll` 为准）。

### 3. `demo_*` 对比

| | `demo_static_dispatch` | `demo_dyn_dispatch` |
|---|------------------------|---------------------|
| 调用 | 两个**不同**的 `process_static::<A/B>` | 两次**同一** `process_dyn`，vtable 参数不同（`@vtable.0` / `@vtable.1`） |
| 符号数 | A、B 各一份 static 特化 | dyn 只有一份 |

## O0 vs O3

本仓库 `ch04_dispatch_O3.ll` 在 `-C opt-level=3` 下生成。因 `#[inline(never)]`，`process_*` 仍保留为独立 `define`；若去掉 `never` 并开 LTO，static 路径更易被 inline 进 `demo_static_dispatch`，dyn 路径仍难消除间接 call。

## 与 HFT

- 热路径 inner loop：IR 里应看到 **直接 `call` 已知符号**（或 inline 后消失），而非 `call i64 %fn_ptr(...)`。
- 排查：`diff ch04_dispatch_O0.ll` 中 `process_dyn` vs 任一 `process_static` 基本块。
