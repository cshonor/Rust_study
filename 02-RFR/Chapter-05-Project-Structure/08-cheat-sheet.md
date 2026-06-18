# 5.2 · 依赖下界 · 速记

← [08 Minimal Dependency Versions](./08-minimal-dependency-versions.md) · [Item 25](../../01-ER/Chapter-04-Dependencies/Item-25-dependency-graph/README.md) · [章索引](./README.md)

---

## 痛点

下界过高 → MVS 解析空间缩小 → 跨 crate 冲突

## 规范

写**测试通过的最低**版本 · 无需求只锁主版本 `1`

## 校验

```bash
cargo update -Z minimal-versions
cargo test -Z minimal-versions
# 或
cargo minimal-versions check --workspace --ignore-private
```

## 库 vs bin

库 **强制** · 二进制可锁 lock / 高版本

## 边界

0.y.z 严锁补丁 · feature 可选 dep 单独测 · workspace 每包独立

## CI

minimal-versions + test 失败禁止合并

## 自测

- [ ] MVS 默认选最高还是最低？  
- [ ] 为何库不应写 `serde = "=1.7.3"`？  
- [ ] minimal-versions 失败说明下界太高还是太低？
