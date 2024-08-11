#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub nickname: String,
    pub password: String
}

pub struct UserCreate {
    pub name: String,
    pub nickname: String,
    pub password: String
}

pub struct UserEdit {
    pub id: u32,
    pub password: String,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub new_password: Option<String>
}

pub struct UserDelete {
    pub id: u32,
    pub password: String
}