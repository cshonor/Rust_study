# 宏核心总览（复习用）

> 第 7 章开篇 · [← 章索引](./README.md)  
> 与 [Rust Reference · Macros](https://doc.rust-lang.org/reference/macros.html) / Book [19.5 宏](../../00-Book/19-advanced-features/19.5-宏.md) 一致。

---

## 1. 定义

**宏 = 编译期元编程**：宏调用在**编译阶段展开**成普通 Rust 代码；**运行时没有「调用宏」这一层**（展开后的代码照常编译、运行）。

一句话：**宏是「写代码的代码」**。

---

## 2. 两大分类

| | **声明宏** | **过程宏** |
|---|------------|------------|
| **定义** | `macro_rules!` | `#[proc_macro_derive]` / `#[proc_macro_attribute]` / `#[proc_macro]` |
| **原理** | **Token 模式匹配** + 模板转录（像结构化 `match`） | Rust 函数处理 **`TokenStream`**（常配合 `syn` / `quote`） |
| **特点** | 入门快、表达力有限 | 强、灵活、可扩展语法 / DSL |
| **例子** | `vec!`、`println!`、`format!` | `#[derive(Serialize)]`、`tokio::main`、类函数宏 |

→ 分节：[01–03 声明宏](./01-when-to-use-declarative-macros.md) · [04–07 过程宏](./04-types-of-procedural-macros.md)

### 过程宏三子类

| 子类 | 形态 | 作用 |
|------|------|------|
| **派生 Derive** | `#[derive(MyTrait)]` | 为 type **追加** `impl` 等 |
| **属性 Attribute** | `#[my_attr(...)]` 修饰项 | 变换函数 / struct / mod 等 |
| **类函数 Function-like** | `my_macro!(...)` | 自定义 token 语法、DSL |

---

## 3. 宏 vs 函数（必背）

| 对比项 | **宏** | **普通函数** |
|--------|--------|--------------|
| **执行时机** | **编译期展开** | **运行期调用** |
| **参数** | 任意 **Token 流**；可变形状 | **固定签名**（类型 + 个数） |
| **能力** | 可生成 `struct` / `impl` / 语句块 | 只执行业务逻辑，**不能**生成新项 |
| **运行开销** | 无「宏调用」开销（已是内联后的代码） | 有调用（通常可内联） |
| **调试** | 报错常指向展开后；需 `cargo expand` | 栈追踪清晰 |
| **可见性** | 须在作用域内**可见**（定义或 `use` 导入） | 同模块可后定义（部分场景） |
| **类型检查** | 展开**之后**才检查 | 定义处即检查 |
| **典型用途** | 样板、可变参数语法、derive、DSL | 逻辑复用、类型安全、日常业务 |

**取舍**：**能用函数（或泛型 + trait）就不用宏** → [06 你真的需要宏吗](./06-so-you-think-you-want-a-macro.md)

---

## 4. FFI + HFT 怎么记

### FFI

| 场景 | 常用宏 |
|------|--------|
| 批量绑定 / 重复 `extern` 形状 | **声明宏** 或 **build.rs + bindgen** |
| `repr(C)` struct、`From`/`Into`、检查布局 | **derive 过程宏**（`bindgen` 生成、`#[derive(...)]` 封装） |
| 固定布局 | 依赖 **`repr(C)`** + [第 2 章 layout](../Chapter-02-Types/02-layout.md)，不靠宏「猜」布局 |

→ [第 11 章 FFI](../Chapter-11-Foreign-Function-Interfaces/README.md)

### HFT

| 要点 | 说明 |
|------|------|
| **零运行时宏开销** | 热路径上展开结果 = 普通代码，适合高频 |
| **别滥用** | 宏难读、难调试；**核心逻辑用函数**，宏只做生成 / 模板 |
| **编译时间** | 过程宏 + 大 derive 拖 **build**；见 [05 代价](./05-cost-of-procedural-macros.md) |

### 一句话取舍

> **函数优先；宏只用于函数做不到的事**（可变 token 形状、生成项、derive）。

---

## 5. 一页极简背诵版

```text
宏 = 编译期展开，运行时无宏调用开销。

两类：
  macro_rules!     → Token 匹配（vec!、println!）
  proc macro       → TokenStream 程序（derive / 属性 / foo!）

过程宏三种：derive · attribute · function-like

宏 vs 函数：编译期 vs 运行期；能生成代码 vs 只能执行逻辑。

FFI：bindgen/derive + repr(C)；HFT：热路径可用展开代码，逻辑仍写函数。
```

---

## 阅读顺序

```text
00（本篇）→ 01 → … → 08 → proc-macro-workshop（可选动手）
```
