# Item 21: Understand what semantic versioning promises

> **Effective Rust** · [Chapter 4 — Dependencies](../../ER-本书目录.md)  
> **中文**：理解语义化版本控制的承诺  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] [WORKSPACE.md MSRV](../../ER-demos/WORKSPACE.md) · [rust-toolchain.toml](../../ER-demos/rust-toolchain.toml)

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| Cargo、`Cargo.toml` | [1.3 Hello Cargo](../../../Book/01-getting-started/1.3-Hello-Cargo.md) |
| 发布、版本不可变、yank | [14.2 发布到 Crates.io](../../../Book/14-cargo-crates/14.2-将crate发布到Crates.io.md) |
| 工作空间与 `Cargo.lock` | [14.3 Cargo 工作空间](../../../Book/14-cargo-crates/14.3-Cargo工作空间.md) |
| trait 对象安全 | [19.2 高级 trait](../../../Book/19-advanced-features/19.2-高级trait.md) |
| 缩小 API 面 | [Item 22](../Item-22-minimize-visibility/README.md)（待补） |
| 通配符导入冲突 | [Item 23](../Item-23-avoid-wildcard-imports/README.md)（待补） |
| CI / Dependabot | [Item 32](../../Chapter-05-Tooling/Item-32-ci/README.md)（待补） |

---

## 一句话

见 [03-key-takeaways.md](./topics/03-key-takeaways.md)。

---

## 专项笔记（按需点开）

| # | 专题 | 阅读 |
|---|------|------|
| 01 | 核心知识点 | [topics/01-core-concepts.md](./topics/01-core-concepts.md) |
| 02 | 逻辑脉络 | [topics/02-logic-flow.md](./topics/02-logic-flow.md) |
| 03 | 重点结论 | [topics/03-key-takeaways.md](./topics/03-key-takeaways.md) |
| 04 | 案例与代码 | [topics/04-examples.md](./topics/04-examples.md) |
| 05 | 易错细节 | [topics/05-pitfalls.md](./topics/05-pitfalls.md) |
| — | 背诵提纲 | [topics/cheat-sheet.md](./topics/cheat-sheet.md) |

---

## 逻辑脉络

```text
Semver 理论：三条规则，清晰明了
         ↓
实践：何谓「向后兼容」充满灰色
         ↓
Hyrum 定律：用户会依赖未文档化的行为
         ↓
作者：少暴露 API + 1.0.0 + deprecated 过渡
用户：合理版本范围 + 定期 MAJOR 升级规划
```

---

## 后续拓展

> 展开版：[ER-拓展索引 § Item 21](../../ER-拓展索引.md#item-21)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---
