#[derive(Clone)]
pub struct Session {
    pub id: u32,
    pub user_id: u32,
    pub expires_at: usize,
}