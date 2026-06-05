use crate::domain::order::OrderEntity;

pub struct UserEntity;

impl UserEntity {
    pub fn new() -> Self {
        let _ = OrderEntity::new();
        Self
    }
}
