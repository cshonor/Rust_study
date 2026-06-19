# 第 1 章 · 安装 LLVM · 速记

← [00 本章定位](./00-overview.md) · [Part 01](../README.md)

---

## 为何源码编译

二进制够用日常 · 源码 = 全开发文件 + 熟悉 CMake + **加后端**

## 环境门槛

C++17 编译器 · Git · CMake ≥3.20 · Ninja · Python ≥3.8 · zlib

磁盘：Debug ~30GB · Release 单平台 ~2GB

## 五步

1. git config  
2. `git clone llvm-project`（Win: `autocrlf=false`）  
3. **外置** `build/`  
4. `cmake -G Ninja -DLLVM_ENABLE_PROJECTS=clang -B build -S llvm`  
5. `cmake --build build -jN` · `check-all` · `install`

## CMake 关键

| 变量 | 用途 |
|------|------|
| `CMAKE_BUILD_TYPE` | Debug/Release/… |
| `LLVM_TARGETS_TO_BUILD` | X86 / all |
| `LLVM_ENABLE_PROJECTS` | clang;lld |
| `LLVM_EXPERIMENTAL_TARGETS_TO_BUILD` | M88k 等 |
| `LLVM_ENABLE_ASSERTIONS` | 开发断言 |
| `LLVM_OPTIMIZED_TABLEGEN` | Debug 时加速 tablegen |

## 本仓库

IR 主线：**clang 版本 + emit=llvm-ir** 即可 · 不必全书源码编译

## 自测

- [ ] 为何禁止在 llvm 源码树内直接 build？  
- [ ] `-DLLVM_ENABLE_PROJECTS=clang` 做什么？  
- [ ] 只读 rustc IR 是否必须源码编译 LLVM？
