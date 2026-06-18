# Package vs Workspace · 速记

← [00 辨析](./00-package-vs-workspace.md) · [03 工作区](./03-workspaces.md) · [章索引](./README.md)

---

## Package vs Crate

1 Cargo.toml = 1 **Package** · 可含多 **crate**（bin/lib/[[bin]]/tests）

`cargo new` → 1 Package · 1 bin · 加 lib.rs → +1 lib crate

## 巨石单 crate

全塞一个 lib/bin · 简单 · 大项目：编译慢 · 边界弱 · 耦合 · feature 臃肿

## exe

永远来自 **bin crate** · 可链接多个 lib crate · 巨石 vs workspace 都有 1 个 exe

## mod vs Package

`mod.rs` = crate 内 · 子目录 `Cargo.toml` = 独立 Package

## 选型

≤几千行单 Package · 上万行/团队 → Workspace

## 自测

- [ ] 改 utils/mod 重编整个 lib 还是只重编 utils？  
- [ ] workspace 下 exe 链接几个 crate？  
- [ ] 为何外人不能只依赖你巨石里一个小工具函数？
