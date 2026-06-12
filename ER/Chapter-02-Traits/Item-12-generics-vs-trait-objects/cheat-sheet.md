# Item 12 · 记忆卡片

← [Item 12 目录](./README.md)

| 要点 | 一句 |
|------|------|
| 单态化 | 编译期一型一份代码；同型跨文件只一份 |
| 并存 | impl 有本体代码；用 dyn 才加 vtable；两路不冲突 |
| vtable | 只存方法**地址**，不存代码；= 楼层指引表 → §09 |
| 泛型 | 静态分发；快；可能膨胀 |
| `dyn` | vtable 动态分发；异构集合；间接开销 |
| 对象安全 | 无泛型方法；无裸 `Self` |
| `Self: Sized` | 保留 dyn，方法仅具体类型可调 |
| 默认 | 先泛型，真要擦除再用 dyn |
| 大白话 | → [06-dispatch-beginner-guide.md](./06-dispatch-beginner-guide.md) |
