# 5.2 Minimal Dependency Versions（依赖下界）

> 所属：**Versioning** · [← 章索引](./README.md)

把依赖写得过紧（只需 `1.5.6` 却写 `1.7.3`）会**不必要收紧**解析空间，与他 crate 冲突。

## 实践

- `Cargo.toml` 写**能通过你测试的最低兼容版本**。
- **`cargo update -Z minimal-versions`**（nightly，以当前 Cargo 为准）探测下界。

ER → [Item 25 依赖图](../../01-ER/Chapter-04-Dependencies/Item-25-dependency-graph/README.md)
