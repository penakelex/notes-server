use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Note {
    pub id: u64,
    pub creator_id: u32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct NoteCreate {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct NoteEdit {
    pub id: u64,
    pub title: Option<String>,
    pub body: Option<String>,
}