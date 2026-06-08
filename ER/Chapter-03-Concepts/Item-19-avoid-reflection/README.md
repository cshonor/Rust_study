# Item 19: Avoid reflection

> **Effective Rust** · [Chapter 3 — Concepts](../ER-本书目录.md)  
> **中文**：避免使用反射  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `dyn Trait`、虚表 | [17.2 trait 对象](../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| DST、`?Sized` | [19.3 高级类型](../../Book/19-advanced-features/19.3-高级类型.md) |
| 泛型 vs trait 对象 | [Item 12](../Chapter-02-Traits/Item-12-generics-vs-trait-objects/README.md) |
| 派生宏代替运行时内省 | [Item 28](../Chapter-05-Tooling/Item-28-macros-judiciously/README.md)（待补） |
| 标记 trait | [Item 10](../Chapter-02-Traits/Item-10-standard-traits/README.md) |

---

## 1. 核心知识点与关键定义

### 反射（Reflection）

- 运行时检查/修改自身结构：查类型、读写字段、调方法。
- Python / Ruby / Java / Go 等常见；**Rust 无完整运行时反射**，也无 C++ 式 **`typeid` / `dynamic_cast` RTTI**。

### `std::any`：有限替代

| API | 作用 |
|-----|------|
| **`type_name::<T>()` / `type_name_of_val`** | 编译期类型名 → **调试/诊断**；非运行时探测 |
| **`TypeId`** | 类型全局唯一 ID → **逻辑里做唯一性校验** |
| **`Any` trait** | 装箱为 `dyn Any`，运行时 **`is` / `downcast_ref` / `downcast_mut`** |

---

## 2. 逻辑脉络

```text
其他语言：反射驱动架构（注册表、插件、ORM…）
         ↓
Rust：编译期单态化 + trait + 宏
         ↓
type_name：编译期泛型 → 对 &dyn Draw 只能看到 Draw，不是 Square
Any：      blanket impl 要求 T: 'static + ?Sized
         ↓
生命周期运行时擦除 → 禁止带非 'static 借用的 Any（防类型混淆）
```

### `type_name` 与 trait 对象

- 传入 `&dyn Draw` → 输出 **`&dyn Draw`**，底层 `Square` 信息已丢。

### `'static` 边界

- `impl<T: 'static + ?Sized> Any for T` — 故意限制；`&'a T` 与 `&'b T` 运行时不可分。

---

## 3. 重点结论与实用要点

### 默认：别做反射架构

- Rust **本来就不支持** —— 最容易遵守的 Item 之一。
- 从 Java/Python 迁来别硬搬「运行时类型表」。

### 替代方案（优先级）

| 需求 | 方向 |
|------|------|
| 行为多态 | **Trait** + `impl` / `dyn Trait` |
| 无法写成方法的约束 | **标记 trait**（marker） |
| 按结构批量 codegen | **derive 宏**（Item 28），编译期生成 |
| 极少数异构集合 | **`Box<dyn Any>`** + downcast（逃生舱） |

### `dyn Any` 使用边界

- 仅当**必须擦除具体类型**且无法用 enum/trait 表达时。
- downcast **只能回到装箱时的原始类型**；不能探查「还实现了哪些 trait」。

---

## 4. 案例与代码要点

### `type_name` 盲区

```rust
let square = Square::new(1, 2, 2);
let draw: &dyn Draw = &square;
println!("{}", std::any::type_name_of_val(&draw));
// → "&dyn reflection::Draw"  不是 Square
```

### downcast 能力上限

```rust
let b: Box<dyn Any> = Box::new(42u32);
assert!(b.is::<u32>());
let n = b.downcast_ref::<u32>().unwrap();
// ❌ 无法问：「它还实现了 Display 吗？」并取出 dyn Display
```

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **解析 `type_name` 字符串** | 非稳定契约；同名路径不保证唯一 → 用 **`TypeId`** |
| **trait 对象 upcasting** | 1.70 前 `dyn Shape`（`Shape: Draw`）不能平滑转 `dyn Draw` — 虚表布局不同 |
| **指望 Any 当通用反射** | 无 supertrait；upcasting 也帮不了 Any |

> **演进注**：Rust **1.76+** 稳定 **trait upcasting**（`dyn Shape` → `dyn Draw`）。但 **`Any` 无父 trait**，新能力**不增加** Any 的反射能力。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 19](../ER-拓展索引.md#item-19)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| Rust | **无**完整反射 / RTTI |
| 设计 | **Trait + 宏**，别 runtime 内省 |
| `type_name` | 调试用；别写业务逻辑 |
| `TypeId` | 要稳定 ID 用这个 |
| `Any` | 稀有逃生舱；仅 downcast 原类型 |
| `'static` | Any 故意不要带借用的 T |
