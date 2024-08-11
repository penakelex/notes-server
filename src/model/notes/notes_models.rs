#[derive(Clone)]
pub struct Note {
    pub id: u64,
    pub creator_id: u32,
    pub title: String,
    pub body: String,
}

pub struct NoteCreate {
    pub title: String,
    pub body: String,
}

pub struct NoteEdit {
    pub id: u64,
    pub title: Option<String>,
    pub body: Option<String>,
}