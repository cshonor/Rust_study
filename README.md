# Rust_study

Rust 学习笔记 + 每章 demo（参考《Rust 程序设计语言》）。

## 目录结构

- **章节目录**：`01-*` ~ `19-*`
- **笔记**：每章/小节为一个 `*.md`
- **demo**：从第 3 章起，按规则 **“一个 md 对应一个独立 Cargo project”**
  - 例：`03-common-concepts/3.3-函数.md` ↔ `03-common-concepts/3.3-functions-demo/`

## 快速开始

### 运行某个 demo

进入对应 demo 目录后运行：

```bash
cargo run
```

示例（函数章节 3.3）：

```bash
cd 03-common-concepts/3.3-functions-demo
cargo run
```

### 猜数字游戏

```bash
cd 02-guessing-game
cargo run
```

## 说明

- 章节目录使用英文命名，避免 Windows 终端乱码。
- `target/` 已在 `.gitignore` 中忽略。
