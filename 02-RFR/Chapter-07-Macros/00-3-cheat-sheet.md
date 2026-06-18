# 00.3 · 宏 vs 函数 · 速记

← [00-3 宏 vs 函数](./00-3-macro-vs-function.md) · [00 hub](./00-macros-overview.md) · [章索引](./README.md)

---

## 八维对比（一句话）

编译期 vs 运行期 · Token vs 固定签名 · 能生成项 vs 不能 · 无调用开销 vs 有 · expand 排错 vs 栈清晰 · 须可见 vs 可后定义 · 展开后类型检查 vs 定义处检查

## 口诀

**能用泛型 + trait + 函数，就不用宏**

## 宏劣势

检查滞后 · 编译慢 · IDE 弱 · 隐藏逻辑

## 必须用宏

批量 impl · DSL · 按字段元编程

## 工具

`cargo expand`

## 自测

- [ ] 为何 `println!` 不能用普通函数替代？  
- [ ] 宏的类型错误为何常难读？  
- [ ] 热路径用宏的真实运行时开销是多少？
