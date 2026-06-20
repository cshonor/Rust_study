# 1.1.1 · `with_` 系列与建造者模式（详细例）

← [01 命名惯例](./01-naming-practices.md) · [01 速记](./01-cheat-sheet.md)

> 建造者模式 → [ER Item 07 Builders](../../01-ER/Chapter-01-Types/Item-07-builders/README.md) · Book [8.1 Vec with_capacity](../../00-Book/08-collections/8.1-vector.md)

---

## 核心思想

**`with_xxx`** 用**方法链**替代「参数很多、顺序易错」的构造函数 — 初始化逻辑**可读、可组合、可省略默认值**。

| 形态 | 典型 |
|------|------|
| **关联函数** | `Vec::with_capacity(n)` — 构造时带一个关键参数 |
| **Builder 链** | `.header(...).timeout(...).send()` — 多步配置再 `build`/`send` |

---

## 例 1：`Vec::with_capacity` — 预分配，少扩容

### 问题

`Vec::new()` 从**空容量**开始。每次 `push` 超出当前容量时：

1. 申请更大堆块  
2. **拷贝**旧元素到新块  
3. 释放旧块  

元素很多时 → **频繁扩容 + 反复拷贝**，性能差。

### 场景：约 100 万元素

```rust
const N: usize = 1_000_000;

// ❌ 未预分配：push 过程中多次扩容（容量常按 ~2 倍增长 → 约 20 次 realloc + 拷贝）
let mut v = Vec::new();
for i in 0..N {
    v.push(i);
}

// ✅ 预分配：一次申请够用的容量，后续 push 不再触发扩容（除非超过 N）
let mut v = Vec::with_capacity(N);
for i in 0..N {
    v.push(i);
}
```

```text
Vec::new()           push × N  →  容量 0→1→2→4→…→2^20  多次 memcpy
Vec::with_capacity(N) push × N  →  容量一次 ≥ N           通常 0 次扩容
```

### 注意

| 点 | 说明 |
|----|------|
| **容量 ≠ 长度** | `with_capacity(N)` 只保证**至少**能无扩容 push N 次；`len()` 仍为 0 |
| **过度预分配** | `with_capacity(1_000_000)` 却只 push 10 个 → 浪费内存 |
| **同类 API** | `String::with_capacity(n)` · `HashMap::with_capacity(n)` · `Vec::reserve(n)`（已有 vec 时追加预留） |

```rust
let mut v = Vec::with_capacity(100);
assert_eq!(v.len(), 0);
assert!(v.capacity() >= 100);
v.push(1);
assert_eq!(v.len(), 1);
```

→ Book [8.1 Vector](../../00-Book/08-collections/8.1-vector.md)

---

## 例 2：HTTP 请求链 — 生态里的 Builder

`reqwest` 没有字面量 `with_`，但**同一思路**：**Client → 方法链配置 → 最后消费**。

```rust
use std::time::Duration;

async fn fetch_api(url: &str, token: &str) -> reqwest::Result<String> {
    let body = reqwest::Client::new()
        .get(url)                                      // 1. 选 HTTP 方法 + URL
        .header("Authorization", format!("Bearer {token}")) // 2. 加头
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))              // 3. 超时
        .send()                                        // 4. 消费 RequestBuilder → 发请求
        .await?
        .text()
        .await?;
    Ok(body)
}
```

### 链上每一步在干什么

```text
Client::new()
  → RequestBuilder（中间状态：还没发出去）
  → .get(url)      再返回 RequestBuilder（可继续配）
  → .header(...)   同上
  → .timeout(...)  同上
  → .send()        消耗 builder，真正建连、发请求 → Response
```

| 对比多参数构造函数 | Builder 链 |
|-------------------|------------|
| `Request::new(method, url, headers, timeout, …)` 顺序难记 | 每步一个方法名，**自文档化** |
| 可选参数用 `None` 占位 | **只写需要的** `.header` / `.timeout` |
| 改一行易牵一发动全身 | 链上**独立追加**配置 |

> 若自己设计 API：可选字段多、组合多 → 优先 **Builder** 或 **`#[derive(Builder)]`**（ER Item 7）。

---

## 例 3：手写 `with_*` Builder（理解机制）

```rust
#[derive(Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
    tls: bool,
}

struct ServerConfigBuilder {
    host: String,
    port: u16,
    workers: usize,
    tls: bool,
}

impl ServerConfigBuilder {
    fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port: 8080,       // 默认值
            workers: 4,
            tls: false,
        }
    }

    fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn with_workers(mut self, n: usize) -> Self {
        self.workers = n;
        self
    }

    fn with_tls(mut self, enable: bool) -> Self {
        self.tls = enable;
        self
    }

    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            workers: self.workers,
            tls: self.tls,
        }
    }
}

fn main() {
    let cfg = ServerConfigBuilder::new("127.0.0.1")
        .with_port(443)
        .with_tls(true)
        .with_workers(8)
        .build();

    assert_eq!(
        cfg,
        ServerConfig {
            host: "127.0.0.1".into(),
            port: 443,
            workers: 8,
            tls: true,
        }
    );
}
```

```text
new("127.0.0.1")     →  host 必填，其余默认
.with_port(443)      →  只改 port，返回 Self 继续链
.with_tls(true)      →  只改 tls
.build()              →  消耗 builder，产出 ServerConfig
```

| 模式 | 说明 |
|------|------|
| **`mut self` + 返回 `Self`** | 消费型链式；每次 `with_*` 移动 builder |
| **`build(self)`** | 最后一步拿走全部配置，builder **不可再用** |
| **默认值在 `new`** | 调用方只写与默认不同的项 |

→ ER [Item 07 示例](../../01-ER/Chapter-01-Types/Item-07-builders/04-examples.md)

---

## 四、`with_` vs 其它命名系列

| 系列 | 阶段 | 所有权 |
|------|------|--------|
| **`with_*`** | **构造 / 配置** | 新建对象或 builder 中间态 |
| **`as_*`** | 已存在对象 | 借用，不消耗 |
| **`into_*`** | 已存在对象 | 消耗，转移 |

---

## 五、何时用

| 场景 | 建议 |
|------|------|
| 已知大致元素个数 | `Vec::with_capacity` / `reserve` |
| 可选字段 ≥ 3、组合多 | `XxxBuilder` + `with_*` |
| 简单 1～2 个参数 | `new` / `with_capacity` 即可，别过度 Builder |

---

## 一句话速记

**`with_` = 构造时「带上这个配置」；链式 Builder = 把复杂初始化拆成可读的一步一步，最后 `build`/`send` 一次落地。**

→ 速记：[01-1-cheat-sheet.md](./01-1-cheat-sheet.md) · 回 [01 命名 §五](./01-naming-practices.md#五with_-系列--构造--配置)
