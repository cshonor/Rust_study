# 4. Conditional Compilation（条件编译）

> [← 章索引](./README.md)

**`#[cfg(...)]`** — 代码或模块仅在条件下进入编译。

## 常见条件

| 类型 | 示例 |
|------|------|
| 平台 | `#[cfg(windows)]`、`target_os = "linux"` |
| 测试 | `#[cfg(test)]` |
| Feature | `#[cfg(feature = "serde")]` |
| 编译器 | `#[cfg(nightly)]`（自定义 cfg 需 rustflags） |

## 依赖侧

`Cargo.toml` 按 target 引入平台专属依赖，与 `cfg` 协同：

```toml
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

Book → [11.1.1 cfg test](../../00-Book/11-testing/11.1.1-cfg-test与模拟输入.md)
