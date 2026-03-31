fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());

    match mode.as_str() {
        "panic" => {
            panic!("crash and burn");
        }
        "oob" => {
            let v = vec![1, 2, 3];
            let _ = v[99];
        }
        _ => {
            eprintln!(
                "usage:\n  cargo run -- panic\n  cargo run -- oob\n\nPowerShell backtrace:\n  $env:RUST_BACKTRACE=1\n  cargo run -- oob"
            );
        }
    }
}

