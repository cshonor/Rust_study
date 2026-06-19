# Chapter 01 — LLVM 环境安装配置

| 项目 | 说明 |
|------|------|
| **学习策略** | **浏览** — IR 路线装二进制即可；源码编译见笔记（Part 04 才必需） |
| **对应书** | 《Learn LLVM 17》第 1 章 |

## 笔记

| 文档 | 内容 |
|------|------|
| [00 本章定位](./00-overview.md) | 与仓库学习取舍 · 阅读路线 |
| [01 环境要求](./01-env-requirements.md) | 硬件 · 编译器 · 工具链 |
| [02 五步构建](./02-build-five-steps.md) | clone → cmake → build → test → install |
| [03 自定义 CMake](./03-custom-cmake-vars.md) | TARGETS · PROJECTS · 进阶开关 |
| [cheat-sheet](./cheat-sheet.md) | 速记 · 自测 |

## 本目录 `notes/`（可选）

记录**本机** LLVM/Clang 版本、PATH、`cargo rustc --emit=llvm-ir` 是否可用 — 几行即可。

## 对照

- [学习取舍](../../Learn-LLVM-17-学习取舍.md) · [Part 01](../README.md) · 下一章 [02 编译器架构](../chapter02_compiler_arch/README.md)
