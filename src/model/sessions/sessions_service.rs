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
    pub async fn create_or_update_session(&self, user_id: u32, validity_days: u16) -> Result<Session> {
        let mut collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::CreateFail))?;

        let previous_session_id = collection.iter()
            .find(|session_option| {
                matches!(session_option, Some(session) if session.user_id == user_id)
            })
            .and_then(|session_option| session_option.as_ref()
                .map(|session| session.id)
            );

        let expiration_time = Utc::now() + Duration::days(validity_days as i64);

        if let Some(id) = previous_session_id {
            let session = collection[id as usize].take().unwrap();
            
            let updated_session = Session {
                expires_at: expiration_time.timestamp() as usize,
                ..session
            };
            
            collection[id as usize] = Some(session.clone());
            
            return Ok(updated_session);
        }
        
        let session = Session {
            id: collection.len() as u32,
            user_id,
            expires_at: expiration_time.timestamp() as usize,
        };

        collection.push(Some(session.clone()));

        Ok(session)
    }

    pub async fn delete_session(&self, user_id: u32) -> Result<()> {
        let mut collection = self.sessions_collection.lock()
            .map_err(|_| Error::Sessions(SessionError::DeleteFail))?;

        let session_id = collection.iter()
            .filter_map(|session_option| {
                match session_option.as_ref() {
                    Some(session)
                    if session.user_id == user_id => Some(session.id),
                    _ => None
                }
            })
            .next();

        if let Some(id) = session_id {
            collection[id as usize].take();
        }

        Ok(())
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