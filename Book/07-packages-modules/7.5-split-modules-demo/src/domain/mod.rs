pub mod user;
pub mod order;

// pub use：提升到 domain 模块出口（外部 crate::domain::UserEntity）
pub use user::UserEntity;
pub use order::OrderEntity;

// 普通 use 若写 use self::user::UserEntity 仅本 mod.rs 有效，与 pub use 同名会冲突
pub fn check() {
    let u = UserEntity::new();
    let o = OrderEntity::new();
    let _ = (u, o);
}
