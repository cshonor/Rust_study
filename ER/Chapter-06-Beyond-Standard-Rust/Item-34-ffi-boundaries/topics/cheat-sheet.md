# Item 34 · 记忆卡片

← [Item 34 目录](../README.md)

| 要点 | 一句 |
|------|------|
| ABI | **`extern "C"`** + **`#[repr(C)]`** |
| 内存 | **谁分配谁释放** |
| 整数 | 固定宽度；慎 `c_int` |
| 封装 | unsafe 在内，**safe API** 在外 |
| Box | **`into_raw` / `from_raw`** |
| 字符串 | **`CString` / `CStr`** |
| panic | **不过 FFI** |
| 绑定 | 手写危险 → **bindgen** |
