# 1.3.2 · `?Sized` 与 `?` — 否定默认泛型约束

← [03-1 Blanket 完整解读](./03-1-ergonomic-blanket-full-guide.md) · [03 速记](./03-cheat-sheet.md)

---

## 核心：`?` = 关掉编译器自动套上的约束

Rust 里每个**未写 bound 的泛型类型参数** `T`，编译器**隐式**加上：

```rust
T: Sized
```

写 **`T: ?Sized`** 不是「新增一条要求」，而是 **否定这条默认规则** — 变成「`T` 可以是 `Sized`，也可以不是（DST）」。

| 写法 | 对 `T` 的含义 |
|------|---------------|
| `T`（默认） | 必须是 **Sized**（编译期已知大小） |
| `T: ?Sized` | **允许** DST：`str`、`[u8]`、`dyn Trait`… |
| `T: Sized` | 显式再强调必须 Sized（与默认等价） |

---

## 和三类公开 Trait **无关**

| 误解 | 正解 |
|------|------|
| `?` 针对「几乎总实现的 Debug / Send…」 | ❌ 与 §02 三类 trait **无关** |
| `?` 关闭「官方默认 trait 实现」 | ❌ 不是 ER Item 13 的 default method |
| `?Sized` 只管 **泛型参数 `T` 的隐式 `Sized`** | ✅ 自己写的 blanket 与 std 一样适用 |

---

## 为何 blanket 里要写 `?Sized`

约束绑在 **泛型参数 `T` 上**，不是绑在 **`&T` 这个包装类型**上。

```rust
impl<T: MyTrait + ?Sized> MyTrait for &T { /* … */ }
//   ^ 这里约束的是 T        ^ impl 的目标类型是 &T
```

### 反例：不加 `?Sized`

```rust
// 隐式 T: Sized
impl<T: MyTrait> MyTrait for &T { /* … */ }

let s: &str = "hi";
// 想让 &str 走 blanket：T = str，但 str 是 DST、!Sized → impl 不适用
```

`&str` 本身是 fat pointer（**Sized**），但被 impl 的 pattern 是 `&T`，其中 **`T = str` 不满足默认 `Sized`** → 编译器拒绝匹配这条 blanket。

加 **`?Sized`** 后：`T` 可以是 `str`，`&str` 就能命中 `impl for &T`。

```rust
impl<T: MyTrait + ?Sized> MyTrait for &T { /* … */ }
// 现在 T = str、T = [u8]、T = dyn MyTrait 均可（若内层类型 impl 了 MyTrait）
```

```text
检查 impl 是否适用时：
  看 T 满不满足 where / 默认 bound  →  不是看 &T 有多大
```

---

## 与 `Sized` trait 本身

| 类型 | `Sized?` |
|------|----------|
| `i32`、`Foo` | ✅ Sized |
| `str`、`[u8]`、`dyn Debug` | ❌ **!Sized**（DST） |
| `&str`、`Box<dyn Trait>` | ✅ 指针/包装本身 Sized |

**`?Sized`**：允许 `T` 落在右列；**包装类型**（`&T`、`Box<T>`）通常仍是 Sized 的 fat/thin pointer。

---

## 速记

```text
?Sized  =  取消「T 必须 Sized」的默认帽子
用途    =  blanket for &T / Box<T> 时兼容 str、切片、trait 对象
无关    =  Debug/Send/Copy 三类 · default method · 公开/私有 trait
```

→ 回 [03-1 §四 包装转发模板](./03-1-ergonomic-blanket-full-guide.md#四形态-b-详解包装转发模板)
