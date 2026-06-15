# 03 · Ownership and Lifetimes · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[02 Data Layout](../02_Data_Layout/README.md) · 下一章：[04 Type Cast](../04_Type_Cast/README.md)

---

官方标题 **Ownership and Lifetimes**。Rust 的灵魂，也是编写 Unsafe 时最需跨越的理论门槛：无 GC 如何保证内存安全，编译器如何处理复杂引用关系。

| 对照 | 路径 |
|------|------|
| RFR 所有权 / 生命周期 | [04-ownership](../../02-RFR/Chapter-01-Foundations/04-ownership.md) · [08-lifetimes](../../02-RFR/Chapter-01-Foundations/08-lifetimes.md) |
| 可变引用与别名 | [06-mutable-references](../../02-RFR/Chapter-01-Foundations/06-mutable-references.md) |
| HRTB | [08-trait-bounds](../../02-RFR/Chapter-02-Types/08-trait-bounds.md) |
| split_at_mut | [19.1 unsafe demo](../../00-Book/19-advanced-features/19.1-unsafe-rust-demo/) |

**读完应能回答**：引用两大法则、型变三分类、何时需要 `PhantomData`、如何安全拆分借用。

---

## 1. 所有权与引用的基础模型

### 替代 GC

所有权系统解决 C/C++ 中悬垂指针、释放后使用等问题，**无需 GC** 即可安全管内存。

### 引用的两大黄金法则

1. 引用**绝不能**长于其 referent 的生命周期。
2. 可变引用 `&mut` **绝不能**被别名化（Aliased）。

→ 源码：[src/ownership.rs](./src/ownership.rs)（字段级拆分借用 — 编译器原生支持）

---

## 2. 别名（Aliasing）的本质与优化

**别名**：多个变量/指针指向重叠内存。

因 Rust 严格限制 `&mut` 别名，编译器可激进优化（寄存器缓存、消除冗余读写、指令重排），而不必担心其它指针暗中改数据——这在许多语言中很难做到。

---

## 3. 生命周期的运作机制

生命周期是强制执行引用规则的**代码区域**（从创建到最后一次使用；**Drop 也算一次使用**）。

| 主题 | 要点 |
|------|------|
| **省略 (Elision)** | 函数签名上三条规则，多数场景不必手写 `'a` |
| **局限性** | 推导有时过粗，会拒绝语义上安全的程序 |
| **无界生命周期** | unsafe 解引用 raw ptr 可能产生无界 `'a`，可膨胀为 `'static`，须尽快用显式边界约束 |
| **HRTB** | `for<'a>` 处理闭包/`Fn` trait 中调用前无法提前命名的生命周期 |

→ 源码：[src/lifetimes.rs](./src/lifetimes.rs)

---

## 4. 子类型与型变（Subtyping and Variance）

为允许「长生命周期降级为短生命周期」传递引用，Rust 引入子类型与型变。**全书最硬核理论之一**：

| 型变 | 含义 | Rust 典型 |
|------|------|-----------|
| **协变 (Covariant)** | 子类型方向与类型参数同向 | `&'a T` 对 `'a` 协变；`&'a T` 对 `T` 多数协变 |
| **逆变 (Contravariant)** | 方向相反 | **函数参数**是 Rust 中主要逆变来源 |
| **不变 (Invariant)** | 不允许升降 | `&mut T` 对 `T` **严格不变**（防 use-after-free） |

→ 源码：[src/variance.rs](./src/variance.rs)（可编译的型变示例 + 注释说明不变性）

---

## 5. 丢弃检查（Drop Check）

确保调用泛型类型的 **Drop** 时，不会访问已销毁的借用数据。

- **基本规则**：泛型参数的生命周期须**严格长于**包含它的类型。
- **`#[may_dangle]`**（unstable）：标准库「逃生舱」，向编译器保证 Drop **绝不**碰可能过期的数据。

→ 笔记级概念；`Vec` 等标准库类型依赖此机制（源码中注释说明，不启用 unstable feature）。

---

## 6. 幽灵数据（PhantomData）

类型在逻辑上「拥有」某泛型/生命周期，但物理字段中**没有**存储它。须用 **`PhantomData`** 零大小标记，向型变与 Drop Check 传递正确语义，避免编译器错误推断。

→ 源码：[src/phantom.rs](./src/phantom.rs)

---

## 7. 借用分离（Splitting Borrows）

借用检查器**原生支持**同一 struct **不同字段**的可变借用。但对数组、切片、树等，检查器无法理解内部**不相交性**。

须用 unsafe（raw ptr）在证明不重叠后拆分可变借用，如 `split_at_mut`、自定义可变迭代器。

→ 源码：[src/split_borrows.rs](./src/split_borrows.rs)
