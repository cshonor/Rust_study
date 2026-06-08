# Item 4 · 案例与代码

← [Item 4 目录](../README.md)

### Newtype + 空 `Error` impl

```rust
#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}
```

### 穷尽枚举 + 溯源

```rust
#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}
```

### `From` + `?`（示意）

```rust
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

fn read_config() -> Result<Config, MyError> {
    let s = std::fs::read_to_string("cfg.toml")?; // 自动 Io → MyError
    Ok(parse(&s)?)
}
```

---
