#[derive(Debug, PartialEq, Eq)]
pub struct Subscriber {
    pub email: String,
}

impl Subscriber {
    pub fn new(email: impl Into<String>) -> Self {
        Self { email: email.into() }
    }
}
