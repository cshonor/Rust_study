# Item 30: Write more than unit tests

> **Effective Rust** · [Chapter 5 — Tooling](../ER-本书目录.md)  
> **中文**：编写除单元测试之外的更多测试  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [item-30-black-box](./demo/)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `#[test]`、`#[cfg(test)]` | [11.1 如何编写测试](../../Book/11-testing/11.1-如何编写测试.md) |
| 单元 / 集成 / `tests/` | [11.3 测试的组织结构](../../Book/11-testing/11.3-测试的组织结构.md) |
| TDD 循环 | [12.4 TDD](../../Book/12-cli-project/12.4-采用测试驱动开发完善库的功能.md) |
| doc test、examples | [Item 27](../Item-27-document-public-api/README.md)、[14.2](../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 类型 vs 测试分工 | [Item 1](../Chapter-01-Types/Item-01-express-data-structures/README.md) |
| 测试里可 panic | [Item 18](../Chapter-03-Concepts/Item-18-dont-panic/README.md) |
| bench / black_box | [Item 20](../Chapter-03-Concepts/Item-20-avoid-over-optimize/README.md) |
| feature / cfg 穷举 | [Item 26](../Chapter-04-Dependencies/Item-26-feature-creep/README.md) |
| CI matrix | [Item 32](../Item-32-ci/README.md)（待补） |

---

## 1. 核心知识点与关键定义

| 类型 | 位置 / 触发 | 可见性 | 用途 |
|------|-------------|--------|------|
| **单元测试** | 同文件 `#[cfg(test)] mod tests` | 可测 **私有** | 白盒、小块逻辑 |
| **集成测试** | 根目录 **`tests/`** | 仅 **pub API** | 黑盒、模块协作 |
| **Doc tests** | `///` 里 code block | pub API | 文档与行为同步 |
| **Examples** | **`examples/`** | pub API | 用户向完整用法 |
| **Benchmarks** | `benches/` 或 bench crate | — | 性能回归 |
| **Fuzz** | **`cargo-fuzz`** (libFuzzer) | — | 随机输入找 crash |

---

## 2. 逻辑脉络

```text
单元测试 ── 内部细节
集成 / doc test ── 边界与契约
examples ── 用户视角整体用法
bench ── 性能（Item 20：先测再优化）
fuzz ── 不可信输入（解析器、网络协议…）
         ↓
类型系统已保证的（Item 1）→ 不必重复测
依赖行为漂移（Item 21）→ 测试作早期预警
         ↓
CI：matrix × features（Item 26/32）；fuzz 单独定期跑
```

---

## 3. 重点结论与实用要点

### 按场景选工具

| 场景 | 工具 |
|------|------|
| 解析外部/不可信输入 | **fuzz**（必须） |
| 性能是核心 SLA | **bench** + 跟踪结果 |
| 修 bug | **先写复现测试**（TDD） |
| 多 feature / 多 target | CI **matrix + cargo-hack**（Item 26） |

### 测试代码 vs 业务代码（Item 18）

- 测试里 **`unwrap` / `expect` / `#[should_panic]`** ✅ — panic = 失败信号。
- **`examples/` 里大量 `unwrap`** ❌ — 用户会复制粘贴；用 `fn main() -> Result<…>` + `?`。

### Bench 注意

- 编译器可能 **const-fold** → `0 ns/iter` → 用 **`black_box`**（§4）。
- 稳定版 bench 支持有限；精确统计用 **`criterion`**（Item 31）。

### Fuzz 与 CI

- Fuzz **开放式、耗时长** → 不适合每次 PR；**定期 / 发布前**跑，维护 **corpus**（§6）。

---

## 4. 案例与代码要点

### `black_box` 防优化

```rust
b.iter(|| {
    let result = factorial(std::hint::black_box(15));
    assert_eq!(result, 1_307_674_368_000);
});
```

无 `black_box` → 编译期算好 → 假快；有 → ~真实 **ns/iter**。

### 集成测试骨架

```rust
// tests/integration_test.rs
use my_crate::public_api;

#[test]
fn smoke() {
    assert!(public_api().is_ok());
}
```

### Doc test（Item 27）

````rust
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
````

`cargo test` 自动编译运行。

---

## 5. 易错细节

| 陷阱 | 说明 |
|------|------|
| **examples 里 unwrap** | 示范坏习惯（Item 18） |
| **只信内置 bench** | 敏感、常需 nightly；用 **criterion** |
| **PR CI 跑 fuzz 数小时** | 成本爆炸；单独 job / 定时 |
| **纯二进制无 lib** | `tests/` 无法 `use main.rs` → 抽 **`lib.rs`**（Book 11.3） |
| **测类型已编码的契约** | 浪费；测集成与 I/O 边界 |

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 30](../ER-拓展索引.md#item-30)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| 单元 | 同文件，可测 private |
| 集成 | `tests/`，仅 pub API |
| doc / examples | 文档测 + 用户向 demo |
| fuzz | 不可信输入 **必做** |
| bench | **`black_box`** / criterion |
| 修 bug | **先写 failing test** |
| 测试 vs 示例 | 测试可 unwrap；**examples 用 `?`** |
