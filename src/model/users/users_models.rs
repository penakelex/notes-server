use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserEdit {
    pub id: u32,
    pub password: String,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Deserialize)]
pub struct UserDelete {
    pub id: u32,
    pub password: String,
}