# 5.1 · MSRV · 速记

← [07 MSRV](./07-msrv.md) · [Item 21 SemVer](../../01-ER/Chapter-04-Dependencies/Item-21-semver/README.md) · [章索引](./README.md)

---

## 定义

最低稳定 Rust 可编译版本 · `rust-version = "1.70"`

## 规则

纯版本号 · ≥ edition 最低 · 低于 MSRV Cargo 报错

## SemVer 生态

抬 MSRV → **minor**（2.6→2.7）· **禁 patch**（2.6.1）

## 落地三步

TOML · README · CI 固定 MSRV · `cargo msrv verify`

## CHANGELOG

新 MSRV · 原因 · 下游影响

## Workspace

各 member 可独立 · 依赖链取**最高** MSRV

## 自测

- [ ] 为何 2.6.1 抬 MSRV 是生态破坏？  
- [ ] MSRV 升级为何通常不算 major？  
- [ ] `cargo-msrv verify` 测什么？
