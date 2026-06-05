pub mod user;
pub mod order;

// pub use：本 mod.rs 内简写 + 对外重导出（不必再单独 use self::）
pub use user::UserEntity;
pub use order::OrderEntity;

pub fn check() {
    let u = UserEntity::new();
    let o = OrderEntity::new();
    let _ = (u, o);
}
