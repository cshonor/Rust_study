# Rust_study

Rust 学习笔记 + 每章 demo。

- **主线**：《Rust 程序设计语言》（The Book），章节在 `Book/01-*`～`Book/19-*`，导航 [`Book/Book-本书目录.md`](Book/Book-本书目录.md)。
- **扩展**：[Effective Rust](https://www.effective-rust.com/)（David Drysdale，35 条建议），本地索引见 [`ER/ER-本书目录.md`](ER/ER-本书目录.md)。

## 目录结构

- **Book（The Book）**：[`Book/Book-本书目录.md`](Book/Book-本书目录.md) + `Book/01-*`～`Book/19-*`（笔记与 demo）
- **ER（Effective Rust）**：`ER/Chapter-01-Types/` … `Chapter-06-Beyond-Standard-Rust/`，每条 Item 一个 `Item-NN-*.md` 占位（与 The Book 并行，按需填写）
- **笔记**：每章/小节为一个 `*.md`
- **demo**：从第 3 章起，按规则 **“一个 md 对应一个独立 Cargo project”**
  - 例：`Book/03-common-concepts/3.3-函数.md` ↔ `Book/03-common-concepts/3.3-functions-demo/`

## 快速开始

### 运行某个 demo（The Book）

进入对应 demo 目录后运行：

```bash
cargo run
```

示例（函数章节 3.3）：

```bash
cd Book/03-common-concepts/3.3-functions-demo
cargo run
```

### 猜数字游戏

```bash
cd Book/02-guessing-game
cargo run
```

### 第 19 章「高级特性」部分 demo（The Book）

```bash
cd Book/19-advanced-features/19.2-advanced-traits-demo
cargo run
```

过程宏 workspace 示例：

```bash
cd Book/19-advanced-features/19.5-macros-demo
cargo run -p decl_macro_demo
cargo run -p pancakes
```

## 说明

- 章节目录使用英文命名，避免 Windows 终端乱码。
- `target/` 已在 `.gitignore` 中忽略。
- **VSCode + rust-analyzer**：见 [`docs/rust-analyzer-VSCode配置.md`](docs/rust-analyzer-VSCode配置.md)；仓库内 [`.vscode/settings.json`](.vscode/settings.json) 为开箱即用工作区配置。
