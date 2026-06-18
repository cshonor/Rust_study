# Package vs Workspace · 速记

← [00 辨析](./00-package-vs-workspace.md) · [03 工作区](./03-workspaces.md) · [章索引](./README.md)

---

## 三层概念

**Package**（一个 Cargo.toml）· **Crate**（lib/bin 编译单元）· **mod**（crate 内模块）

## 单 Package

`lib.rs` + `main.rs` = **2 crate，1 Package** · main 自动 `use 包名::`

## Workspace

多 Package · 各子目录自有 Cargo.toml · 根 `[workspace] members`

## mod vs crate

| | mod | 独立 crate |
|---|-----|------------|
| 标志 | `mod.rs` 在 src 下 | 子目录 **Cargo.toml** |
| 场景 | 单 Package 内文件夹 | Workspace 子包 |

## 选型

小项目 → 单 Package lib+main · 大项目分层 → Workspace

## 自测

- [ ] `cargo new` 默认几个 crate？加 lib.rs 后几个？  
- [ ] `db/mod.rs` 在单 Package 里是 crate 还是 mod？  
- [ ] Workspace 根 Cargo.toml 为何常不写 dependencies？
