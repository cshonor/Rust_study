# 1.2.1 · 通用标准 Trait 完整解读（Unsurprising）

← [02 通用 Trait](./02-common-traits-for-types.md) · [02 速记](./02-cheat-sheet.md)

> **准则**：对外公开类型尽量**开箱即用**，实现用户预期的基础 Trait。  
> ER → [Item 10 标准 trait](../../01-ER/Chapter-02-Traits/Item-10-standard-traits/README.md) · Book → [10.2 trait · derive](../../00-Book/10-generics-traits-lifetimes/10.2-trait.md)

---

## 三大类一览

| 类别 | Trait | 策略 |
|------|-------|------|
| **几乎总是该有** | `Debug` · `PartialEq` / `Eq` | 公开类型默认加上 |
| **多线程默认假设** | `Send` · `Sync` | 多数自动满足；不能满足须文档说明 |
| **谨慎对待** | `Copy` · `Hash` | 有副作用或长期约束，按需 |

---

## 一、几乎总是该实现

### 1. `Debug`

**作用**：支持 `{:?}` / `{:#?}` — 日志、调试、单元测试断言失败时的自动输出。**任何公开类型都应提供**，哪怕手动写简化版。

```rust
#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let u = User { id: 1, name: "Alice".into() };
    println!("{:?}", u); // User { id: 1, name: "Alice" }
    assert_eq!(u.id, 2, "用户信息：{u:?}"); // 断言失败会打印 Debug
}
```

**手动实现** — 隐藏敏感字段：

```rust
struct Secret {
    token: String,
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret")
            .field("token", &"******")
            .finish()
    }
}
```

---

### 2. `PartialEq` / `Eq`

| Trait | 作用 |
|-------|------|
| **`PartialEq`** | `==` / `!=`；允许「部分相等」（如浮点 `NaN`） |
| **`Eq`** | 标记**全序相等** — 自反、对称、传递；**无 NaN 式例外** |

```rust
#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let u1 = User { id: 1, name: "Alice".into() };
    let u2 = User { id: 1, name: "Alice".into() };
    let u3 = User { id: 2, name: "Bob".into() };

    assert!(u1 == u2);
    assert!(u1 != u3);
}
```

**作 `HashMap` / `HashSet` 键** — 还须 `Hash`（见第三节）：

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct UserKey {
    id: u64,
    name: String,
}

let mut map = HashMap::new();
let key = UserKey { id: 1, name: "Alice".into() };
map.insert(key, "管理员");
```

**浮点特例**：`f64` 只能 `PartialEq`，**不能** `Eq`（`NaN != NaN`，违反自反性）。

---

## 二、异步 / 多线程默认假设：`Send` + `Sync`

| Trait | 含义 |
|-------|------|
| **`Send`** | 所有权可安全**转移到**其他线程 |
| **`Sync`** | **`&T`** 可安全跨线程共享（`T: Sync` ⇔ `&T: Send`） |

**规则**：

- 无内部可变裸指针、无 `Rc`/`Cell` 等线程不安全包装 → 多数类型**自动** `Send + Sync`；
- 若**不能**实现 → 文档写明原因 + 替代（如 `Rc` → `Arc`）。

### 示例 1：默认安全

```rust
#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

fn test_send() {
    let u = User { id: 1, name: "test".into() };
    std::thread::spawn(move || {
        println!("{u:?}");
    })
    .join()
    .unwrap();
}
```

### 示例 2：`Rc` 非 `Send`

```rust
use std::rc::Rc;

let data = Rc::new(123);
// ❌ Rc 不实现 Send
// std::thread::spawn(move || println!("{data}"));

use std::sync::Arc;
let safe = Arc::new(123);
std::thread::spawn(move || println!("{safe}")).join().unwrap();
```

---

## 三、谨慎对待

### 1. `Copy` ⚠️

**约束**：

- 实现 `Copy` 后，将来加 `String` / `Vec` 等非 `Copy` 字段 → **编译失败**（API 演进被锁死）；
- 赋值、传参**隐式复制**，大结构可能带来性能/语义意外。

**仅推荐**：无堆分配、稳定小值 — `i32`、`Point { x, y: f32 }`、简单 C 风格枚举。

```rust
// ✅ 适合 Copy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: f32,
    y: f32,
}

// ❌ 含 String 无法 derive Copy
// #[derive(Copy)]
// struct User { id: u64, name: String }  // 编译错误
```

**底层**：`Copy: Clone` — 复制成本极低；实现后 move 语义在赋值/传参处变为 bitwise 拷贝。

---

### 2. `Hash` ⚠️

**规则**：`Hash` 必须与 `PartialEq` / `Eq` **完全一致** — 相等 ⇒ 哈希相同；否则 `HashMap` 查找异常。

```rust
#[derive(Debug, PartialEq, Eq, Hash)]
struct User {
    id: u64,
    name: String,
}

use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(User { id: 1, name: "Alice".into() });
```

**危险场景** — 键入集合后**修改参与哈希的字段**：

```rust
#[derive(Debug, PartialEq, Eq, Hash)]
struct BadKey {
    id: u64,
    name: String,
}

let mut set = HashSet::new();
let mut key = BadKey { id: 1, name: "Alice".into() };
set.insert(key.clone());
key.name = "Bob".into(); // 改了字段，但 set 里仍按旧哈希存着 → 逻辑上「丢键」
```

**对策**：哈希键类型**不暴露**可随意 mut 的字段；或键用不可变 newtype / 只读视图。

---

## 四、可直接复制的标准模板

### A. 通用公开结构体（默认首选）

```rust
/// 对外 API 的常规值类型：Debug + 相等，暂不 Copy/Hash。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget {
    id: u64,
    label: String,
}
```

### B. 需要作 `HashMap` / `HashSet` 键

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetId {
    id: u64,
    namespace: String,
}
```

### C. 纯栈小值（可 Copy）

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}
```

### D. 含敏感字段（手动 Debug）

```rust
pub struct ApiToken {
    inner: String,
}

impl std::fmt::Debug for ApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiToken").field("inner", &"<redacted>").finish()
    }
}

impl Clone for ApiToken { /* … */ }
// PartialEq / Eq 按业务决定是否派生
```

### E. 非 `Send` 类型（须在文档说明）

```rust
use std::rc::Rc;

/// 单线程共享；**非 Send**。跨线程请用 `Arc<Inner>` 或克隆数据。
pub struct SharedCounter {
    inner: Rc<CounterState>,
}
```

---

## 五、开发落地清单

1. 对外 struct / enum → **`#[derive(Debug, PartialEq, Eq)]`** 起步；
2. 多线程 crate → 默认假设 `Send + Sync`；例外写进 rustdoc；
3. **`Copy`** 只给稳定小值；含堆类型一律 **`Clone`** 不 **`Copy`**；
4. 要当 map/set 键 → 加 **`Hash`**，且键字段**不可在入集合后静默修改**；
5. 敏感数据 → **手动 `Debug`** 脱敏。

---

→ 速记：[02-1-cheat-sheet.md](./02-1-cheat-sheet.md) · 回 [02 通用 Trait](./02-common-traits-for-types.md)
