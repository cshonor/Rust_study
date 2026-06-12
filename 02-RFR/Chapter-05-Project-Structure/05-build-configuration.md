# 3.2 Build Configuration（构建配置）

> 所属：**Project Configuration** · [← 章索引](./README.md)

## `[patch]` — 临时替换依赖

上游 bug → fork / 本地 fix → **`[patch]`** 优先 **path / git revision**，验证后再 upstream。

## `[profile.*]` — 优化权衡

| 选项 | 直觉 |
|------|------|
| **`opt-level`** | `0` 调试；`3` 速度；`"s"`/`"z"` 体积 |
| **`codegen-units`** | 多 → 编译快、单单元优化弱；release 常减小换峰值性能 |
| **`lto`** | 链接期跨单元优化；运行时收益大，**编译慢** |

## `panic = "abort"`

- 不展开栈；体积与嵌入式友好。
- **全局策略** — 与团队 / 平台约定一致。

## 成员 / 继承

Workspace 根 `[profile.*]` 可被成员 **inherit**（Cargo 新版本特性，以文档为准）。
