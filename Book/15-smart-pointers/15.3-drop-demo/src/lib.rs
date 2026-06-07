//! 15.3 Drop / RAII demo

use std::mem::{self, ManuallyDrop};
use std::sync::Mutex;

/// 带日志的 RAII 标签，观察 drop 顺序。
#[derive(Debug)]
pub struct Tag(pub &'static str);

impl Drop for Tag {
    fn drop(&mut self) {
        println!("  drop Tag({})", self.0);
    }
}

/// 书版 CustomSmartPointer
pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("  Dropping CustomSmartPointer `{}`", self.data);
    }
}

struct Inner;
struct Outer(Inner);

impl Drop for Inner {
    fn drop(&mut self) {
        println!("  drop Inner");
    }
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("  drop Outer");
    }
}

struct CountDrop(u32);

impl Drop for CountDrop {
    fn drop(&mut self) {
        println!("  drop E({})", self.0);
    }
}

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  Resource freed");
    }
}

/// 带 Drop 日志的字段，观察「自定义 drop 之后字段才 drop」
struct FieldTag(&'static str);

impl Drop for FieldTag {
    fn drop(&mut self) {
        println!("  字段 {} 内存回收", self.0);
    }
}

/// §二 模拟 FileHandle：外部资源 + 内存字段
pub struct FileHandle {
    pub fd: i32,
    buf: FieldTag,
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("  手动执行：关闭 fd = {}（外部资源）", self.fd);
    }
}

pub fn demo_custom_drop_then_fields() {
    let _fh = FileHandle {
        fd: 10,
        buf: FieldTag("buf/Vec"),
    };
    println!("  FileHandle 创建完成，即将出作用域");
}

/// §三 作用域结束自动 drop + §五 LIFO 顺序
pub fn demo_scope_and_order() {
    println!("  --- 进入内层作用域 ---");
    {
        let _f = Tag("block");
    }
    println!("  --- 内层作用域结束 ---");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("  CustomSmartPointers created（离开函数时先 d 后 c）");
    let _ = (c, d);
}

/// §三 mem::drop 提前释放
pub fn demo_mem_drop_early() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("  CustomSmartPointer created");
    mem::drop(c);
    println!("  mem::drop 之后（c 已不可用）");
}

/// §15.3.1 嵌套：Outer → Inner
pub fn demo_nested_drop() {
    let _o = Outer(Inner);
    println!("  Outer(Inner) 即将出作用域");
}

/// §15.3.1 Vec 元素正序 drop
pub fn demo_vec_drop_order() {
    let v = vec![
        CountDrop(1),
        CountDrop(2),
        CountDrop(3),
    ];
    println!("  Vec 含 3 元素，即将 drop");
    drop(v);
}

/// §四 MutexGuard RAII 解锁（Drop 无 println，仅演示编译通过）
pub fn demo_mutex_guard_drop() {
    let m = Mutex::new(0_i32);
    {
        let mut guard = m.lock().unwrap();
        *guard += 1;
        println!("  持锁中: value = {}", *guard);
    }
    let after = m.lock().unwrap();
    println!("  锁已释放，可再次 lock: value = {}", *after);
}

/// §15.3.1 ManuallyDrop
pub fn demo_manually_drop() {
    let mut slot = ManuallyDrop::new(Resource);
    println!("  ManuallyDrop 包裹 Resource（尚未 drop）");
    let taken = unsafe { ManuallyDrop::take(&mut slot) };
    drop(taken);
    println!("  手动 take + drop 完成");
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex as StdMutex;

    static DROP_LOG: StdMutex<Vec<&'static str>> = StdMutex::new(Vec::new());

    struct LogOnDrop(&'static str);

    impl Drop for LogOnDrop {
        fn drop(&mut self) {
            DROP_LOG.lock().unwrap().push(self.0);
        }
    }

    #[test]
    fn drop_order_lifo() {
        {
            let mut log = DROP_LOG.lock().unwrap();
            log.clear();
        }
        {
            let _a = LogOnDrop("a");
            let _b = LogOnDrop("b");
        }
        let log = DROP_LOG.lock().unwrap();
        assert_eq!(*log, vec!["b", "a"]);
    }

    #[test]
    fn vec_drop_reverse() {
        {
            let mut log = DROP_LOG.lock().unwrap();
            log.clear();
        }
        struct E(u32);
        impl Drop for E {
            fn drop(&mut self) {
                DROP_LOG.lock().unwrap().push(match self.0 {
                    1 => "E1",
                    2 => "E2",
                    3 => "E3",
                    _ => "?",
                });
            }
        }
        drop(vec![E(1), E(2), E(3)]);
        let log = DROP_LOG.lock().unwrap();
        assert_eq!(*log, vec!["E1", "E2", "E3"]);
    }
}

// 编译失败示例（文档用，勿取消注释）：
// fn explicit_drop_disallowed() {
//     let mut f = Tag("x");
//     f.drop(); // error: explicit call to destructor `drop` is disallowed
// }
