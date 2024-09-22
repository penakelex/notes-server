#[derive(Clone, Debug)]
pub struct AuthTokenContext {
    user_id: u32,
}

impl AuthTokenContext {
    pub fn new(user_id: u32) -> Self {
        Self { user_id }
    }
}

impl AuthTokenContext {
    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}