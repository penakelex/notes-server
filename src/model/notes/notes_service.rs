use std::sync::{Arc, Mutex};

use crate::error::{Error, NoteError, Result};
use crate::model::notes::notes_models::{Note, NoteCreate, NoteEdit};

#[derive(Clone)]
pub struct NotesService {
    notes_collection: Arc<Mutex<Vec<Option<Note>>>>,
}

impl NotesService {
    pub fn new() -> Self {
        Self { notes_collection: Arc::default() }
    }
}

impl NotesService {
    pub async fn create_note(
        &self, note_create: NoteCreate, creator_id: u32,
    ) -> Result<Note> {
        let mut collection = self.notes_collection.lock()
            .map_err(|_| Error::Notes(NoteError::CreateFail))?;

        let id = collection.len() as u64;

        let note = Note {
            id,
            creator_id,
            title: note_create.title,
            body: note_create.body,
        };

        collection.push(Some(note.clone()));

        Ok(note)
    }

    pub async fn list_of_notes(&self, creator_id: u32) -> Result<Vec<Note>> {
        let collection = self.notes_collection.lock()
            .map_err(|_| Error::Notes(NoteError::ReceiveFail))?;

        let notes = collection.iter().filter_map(
            |note_option| note_option.as_ref().and_then(
                |note| if note.creator_id == creator_id {
                    Some(note.clone())
                } else {
                    None
                }
            )
        ).collect::<Vec<Note>>();

        Ok(notes)
    }

    pub async fn edit_note(
        &self, note_edit: NoteEdit, editor_id: u32,
    ) -> Result<Note> {
        let mut collection = self.notes_collection.lock()
            .map_err(|_| Error::Notes(NoteError::EditFail))?;

        let note = collection.get_mut(note_edit.id as usize)
            .user_can_change_note(editor_id, Error::Notes(NoteError::EditorCanNotEditNote))?
            .and_then(|note| note.take())
            .ok_or(Error::Notes(NoteError::NoteDoesNotExists))?;

        let edited_note = Note {
            title: note_edit.title.unwrap_or(note.title),
            body: note_edit.body.unwrap_or(note.body),
            ..note
        };

        collection[edited_note.id as usize] = Some(edited_note.clone());

        Ok(edited_note)
    }

    pub async fn delete_note(&self, note_id: u64, deleter_id: u32) -> Result<Note> {
        let mut collection = self.notes_collection.lock()
            .map_err(|_| Error::Notes(NoteError::DeleteFail))?;

        let deleted_note = collection.get_mut(note_id as usize)
            .user_can_change_note(deleter_id, Error::Notes(NoteError::DeleterCanNotDeleteNote))?
            .and_then(|note| note.take())
            .ok_or(Error::Notes(NoteError::NoteDoesNotExists))?;

        Ok(deleted_note)
    }
}

trait NoteChanger: Sized {
    fn user_can_change_note(self, changer_id: u32, error: Error) -> Result<Self>;
}

impl NoteChanger for Option<&mut Option<Note>> {
    fn user_can_change_note(self, changer_id: u32, error: Error) -> Result<Self> {
        match self.as_ref() {
            None => Err(error),
            Some(note) => {
                let is_user_can_change = note.as_ref()
                    .is_some_and(move |note| note.creator_id != changer_id);
                if is_user_can_change {
                    Ok(self)
                } else {
                    Err(error)
                }
            }
        }
    }
}