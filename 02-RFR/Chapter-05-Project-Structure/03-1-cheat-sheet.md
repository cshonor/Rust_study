# Cargo.lock · 速记

← [03-1 Cargo.lock](./03-1-cargo-lock.md) · [03 工作区](./03-workspaces.md) · [章索引](./README.md)

---

## 是什么

自动生成 · TOML=范围 · lock=精确版本 · 可复现构建

## 提交

| 应用/bin | **提交** lock |
| 纯 lib | **不提交**（下游忽略） |
| Workspace | **根目录唯一** lock |

## 字段

`[[package]]` · name · version（精确）· source · checksum · dependencies[]

根项目块无 source/checksum

## 命令

`cargo add` 改 lock · `cargo update` / `-p pkg` 升级 · `build --locked` CI 严格

## 误区

❌ 手改 lock · 应用不提交 · 纯 lib 提交 · 每 member 各自 lock

## demo

[`cargo-lock-demo/`](./cargo-lock-demo/) — `serde = "1.0"` → 7 包传递树

## 自测

- [ ] TOML `"1.0"` 和 lock `1.0.86` 各表示什么？  
- [ ] 纯库为何提交 lock 无助于下游？  
- [ ] `--locked` 与 `cargo update` 相反效果是什么？
