use std::sync::{Arc, Mutex};
use chrono::{Duration, Utc};
use crate::model::sessions::sessions_models::Session;
use crate::error::{Result, Error, SessionError};

#[derive(Clone)]
pub struct SessionsService {
    sessions_collection: Arc<Mutex<Vec<Option<Session>>>>,
}

impl SessionsService {
    pub fn new() -> Self {
        Self { sessions_collection: Arc::default() }
    }
}

impl SessionsService {
    pub async fn create_session(&self, user_id: u32, validity_days: u16) -> Result<Session> {
        let mut collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::CreateFail))?;

        let expiration_time = Utc::now() + Duration::days(validity_days as i64);

        let session = Session {
            id: collection.len() as u32,
            user_id,
            expires_at: expiration_time.timestamp() as usize,
        };

        collection.push(Some(session.clone()));

        Ok(session)
    }

    pub async fn update_session(&self, session_id: u32, validity_days: u16) -> Result<Session> {
        let mut collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::UpdateFail))?;

        let previous_session = collection.get_mut(session_id as usize)
            .ok_or(Error::Sessions(SessionError::SessionDoesNotExists))?
            .take()
            .ok_or(Error::Sessions(SessionError::SessionDoesNotExists))?;

        let expiration_time = Utc::now() + Duration::days(validity_days as i64);

        let new_session = Session {
            expires_at: expiration_time.timestamp() as usize,
            ..previous_session
        };

        collection[session_id as usize] = Some(new_session.clone());

        Ok(new_session)
    }

    pub async fn delete_session(&self, session_id: u32) -> Result<Session> {
        let mut collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::DeleteFail))?;

        let session = collection.get_mut(session_id as usize)
            .ok_or(Error::Sessions(SessionError::SessionDoesNotExists))?
            .take();

        session.ok_or(Error::Sessions(SessionError::SessionDoesNotExists))
    }

    pub async fn session_validity(
        &self, session_id: u32, token_expires_at: usize,
    ) -> Result<u32> {
        let collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::ValidityCheckFail))?;

        let session = collection[session_id as usize].as_ref()
            .ok_or(Error::Sessions(SessionError::SessionDoesNotExists))?;
        
        if session.expires_at == token_expires_at {
            Ok(session.user_id)
        } else {
            Err(Error::Sessions(SessionError::SessionInvalid))
        }
    }
}