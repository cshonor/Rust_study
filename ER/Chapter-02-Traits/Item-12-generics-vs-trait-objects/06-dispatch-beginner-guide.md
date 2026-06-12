# Item 12 · 单态化与静/动态分发（新手大白话）

← [Item 12 目录](./README.md) · 闭包 `Fn*` → [Item 2 §01](../../Chapter-01-Types/Item-02-express-common-behavior/01-core-concepts.md)

结合 **泛型、单态化、静态分发、动态分发、虚表（vtable）** 一次讲清，尽量不用术语堆叠。

---

## 1. 单态化（Monomorphization）

一句话：**编译时把泛型里的「模糊类型 T」，换成你实际用到的「具体类型」，并为每种具体类型生成专属代码。**

例子：写了一个泛型函数 `fn f<T>(x: T) { ... }`

| 调用情况 | 编译器行为 |
|----------|------------|
| 全项目多处都用 **同一个** `T`（比如都是 `i32`） | **只生成 1 份** `f_i32` 机器码，跨 `.rs` 文件共用 |
| 有的用 `i32`，有的用 `String` | **每种类型各 1 份**：`f_i32`、`f_String` |

特点：

- 发生在**编译阶段**——运行时不用再判断「这次 T 是谁」
- **快**，无额外类型判断开销
- 缺点：具体类型太多 → 二进制变大（**代码膨胀** / code bloat）

### 跨文件疑问（a.rs + b.rs 都调同一泛型）

```text
a.rs 和 b.rs 都调用 f::<MyStruct>()
  → 单态化只生成一份 f_MyStruct
  → 两个文件链接到同一份机器码，不会重复生成

a.rs 用 MyStruct，b.rs 用 OtherStruct
  → 各生成一份
```

---

## 2. 静态分发（Static Dispatch）

**静态分发 = 靠单态化实现的调用方式。**

流程：

1. **编译期**就确定：这次调用最终跳到哪一段具体代码
2. **运行时**直接执行，**不用查表**

常见场景：

- 泛型 + `impl Trait` / trait bound（`fn draw<T: Draw>(x: &T)`）
- 闭包走泛型参数（`F: FnOnce()`）——每个闭包类型编译期特化

✅ HFT / 高性能路径首选：可内联、无 vtable 间接跳转。

---

## 3. 动态分发（Dynamic Dispatch）与虚表（vtable）

### （1）虚表 vtable 是什么？

一张**方法地址清单**。

用 `dyn Trait`（trait 对象）时：

- 编译器为「实现了该 trait 的具体类型」准备 vtable
- 表里存着各 trait 方法对应的代码地址

### （2）动态分发怎么工作？

**运行时才查表、再跳转执行：**

1. 程序跑起来，通过**胖指针**（数据指针 + vtable 指针）找到 vtable
2. 查表得到方法地址，间接调用

常见场景：

- `&dyn Trait` / `Box<dyn Trait>`
- 需要**同一容器里放多种不同类型**（如 `Vec<Box<dyn Handler>>`）

⚠️ 比静态分发多一次间接跳转；换来**运行时灵活**、二进制更小（一份 `dyn` 调用逻辑服务多种类型）。

---

## 4. 静态 vs 动态：一张表

| | 静态分发 | 动态分发 |
|---|----------|----------|
| 机制 | 单态化 | vtable |
| 何时定调用目标 | **编译期** | **运行期** |
| 写法 | 泛型、`impl Trait`、闭包 `F: Fn*` | `dyn Trait`、`Box<dyn Trait>` |
| 速度 | 快，可内联 | 略慢（查表） |
| 灵活性 | 异构集合麻烦 | 异构集合方便 |
| 二进制 | 类型多会膨胀 | 通常更省 |

---

## 5. 闭包三兄弟 Fn / FnMut / FnOnce（复习）

描述闭包**怎么用外部环境**：

| Trait | 能力 | 调用 |
|-------|------|------|
| **`Fn`** | 只读环境 | 可反复调用 |
| **`FnMut`** | 可改环境 | 可反复调用 |
| **`FnOnce`** | 可消耗环境 | 通常只能一次 |

**向下兼容（要求弱 → 强的都能传）：**

```text
要求 FnOnce  →  Fn / FnMut / FnOnce  都能传
要求 FnMut   →  FnMut / Fn  能传；FnOnce 不行
要求 Fn      →  只能 Fn
```

记忆：**Fn 能力最强（只读），FnOnce 最弱（可消耗）**；API 参数写 **`F: FnOnce`** 最宽，写死 `Fn` 最窄。

闭包走**泛型 + 静态分发**时，每个闭包类型各单态化一份；走 **`Box<dyn Fn()>`** 则**动态分发**。

---

## 6. 最简记忆口诀

1. **单态化**：编译期泛型 → 具体类型；一型一份代码，同型跨文件复用
2. **静态分发**：编译定好调用地址，跑得快，靠泛型 / `impl Trait`
3. **动态分发**：运行查 vtable，灵活稍慢，靠 `dyn Trait`
4. **闭包 Fn\***：Fn > FnMut > FnOnce（能力）；要求 FnOnce 时三者皆可，要求 Fn 时只能 Fn

## 相关

- 形式化定义 → [01-core-concepts.md](./01-core-concepts.md)
- 选型结论 → [03-key-takeaways.md](./03-key-takeaways.md)
- Item 2 闭包与 `'env` → [Item 2 README](../../Chapter-01-Types/Item-02-express-common-behavior/README.md)
