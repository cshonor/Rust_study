# Item 1 · 背诵提纲

← [Item 1 目录](../README.md)

考前 / 面试前 3 分钟速览。

## 四句话总览（默写版）

1. **基础类型**：位数明确、强类型、整数 / `char` **不隐式转换**。
2. **聚合**：数组同构定长，元组异构定长，结构体命名字段。
3. **enum**：互斥变体 + 可带数据 = **ADT**。
4. **Option / Result**：无 null；错误必须显式处理。

## 速查表

| 块 | 背一句 |
|----|--------|
| 整数 | `i32`/`u64` 等**定长**；非 Python 自动扩容 int |
| 布局 | 固定大小 → 结构体紧凑、缓存友好（HFT） |
| 比较 | 异类型不能 `==`；先 `as` / `From` **间接比较** |
| 溢出 | debug panic；release 默认环绕 → 用 `checked_*` / `wrapping_*` 显式定语义 |
| 标量 | `bool` · `f32/f64` · `()` · `char`（4B Unicode 标量） |
| `bool` | 1 字节；`true`≈`0x01`、`false`≈`0x00`；≠ 全 1 / 全 0 |
| `()` | 无可用返回值；无返回类型或语句结尾 `;` → 返回 `()` |
| 数组 | `[T; N]` 编译期定长同构；越界 panic |
| 元组 | 定长异构；`.0` `.1` |
| struct | 命名字段；元组结构体用 `.0` |
| ADT | 单元 / 元组 / 结构体变体，互斥 |
| `Option` | `Some` / `None` → 无 null |
| `Result` | `Ok` / `Err` → 可恢复错误 |

## Item 1 设计主张

| 要点 | 一句 |
|------|------|
| 核心主张 | 设计写进类型，无效状态编译不过 |
| ADT | `enum` 变体可带数据 |
| 哨兵 | 用 `Option`，别用魔法值 |
| 失败 | 用 `Result`，别用特殊返回码 |
| `match` | 穷尽；公开 enum 加变体要小心破坏性 |

## 专题索引

| # | 文件 |
|---|------|
| 01 | [fundamental-types](./01-fundamental-types.md) |
| 02 | [scalar-types](./02-scalar-types.md) |
| 03 | [aggregate-types](./03-aggregate-types.md) |
| 04 | [enum-adt](./04-enum-adt.md) |
| 05 | [option-result](./05-option-result.md) |
| 06 | [design-patterns](./06-design-patterns.md) |
| 07 | [pitfalls](./07-pitfalls.md) |
