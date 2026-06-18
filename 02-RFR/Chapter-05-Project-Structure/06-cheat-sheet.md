# 4 · 条件编译 · 速记

← [06 Conditional Compilation](./06-conditional-compilation.md) · [02 Feature cfg](./02-using-features-in-crate.md) · [章索引](./README.md)

---

## cfg vs if

`#[cfg]` 不满足 → **删代码** · `if` 两分支都编译

## 三大 Predicate

平台 `target_*` · `test` · `feature = "x"`

## 组合

`all()` · `any()` · `not()`

## 三工具

| | |
|---|---|
| `#[cfg]` | 删 item |
| `#[cfg_attr]` | 条件属性 |
| `cfg!()` | 保留代码，编译期 bool |

## Cargo 侧

```toml
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

## 避坑

feature 收 mod · CI 双构建 · 平台 dep 别全局引 · `--cfg` 非 feature

## 自测

- [ ] `cfg!(windows)` 和 `#[cfg(windows)] fn f()` 差在哪？  
- [ ] 为何 `#[cfg(test)] mod tests` 不进 release 二进制？  
- [ ] 自定义 `nightly` cfg 怎么注入？
