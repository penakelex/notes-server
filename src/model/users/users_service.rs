use std::sync::{Arc, Mutex};
use crate::model::users::users_models::{User, UserCreate, UserEdit, UserLogin};
use crate::error::{Result, Error, UserError};

#[derive(Clone)]
pub struct UsersService {
    users_collection: Arc<Mutex<Vec<Option<User>>>>,
}

impl UsersService {
    pub fn new() -> Self {
        Self { users_collection: Arc::default() }
    }
}

impl UsersService {
    pub async fn create_user(&self, user_create: UserCreate) -> Result<User> {
        let mut collection = self.users_collection.lock()
            .map_err(|_| Error::User(UserError::RegisterFail))?;

        let is_user_with_same_nickname_exists = collection.iter()
            .any(|user_option| user_option.as_ref()
                .is_some_and(|user|user.nickname == user_create.nickname)
            );

        if is_user_with_same_nickname_exists {
            return Err(Error::User(UserError::RegisterFailNicknameCaptured));
        }

        let id = collection.len() as u32;

        let user = User {
            id,
            name: user_create.name,
            nickname: user_create.nickname,
            password: user_create.password,
        };

        collection.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn edit_user(&self, user_edit: UserEdit) -> Result<User> {
        let mut collection = self.users_collection.lock()
            .map_err(|_| Error::User(UserError::EditFail))?;

        if user_edit.name.is_none()
            && user_edit.nickname.is_none()
            && user_edit.new_password.is_none()
        {
            return Err(Error::User(UserError::EmptyFieldToEdit));
        }

        if let Some(new_nickname) = user_edit.nickname.as_ref() {
            let is_new_nickname_captured = collection.iter()
                .any(|user_option| user_option.as_ref()
                    .is_some_and(|user| user.nickname == *new_nickname
                        && user.id != user_edit.id
                    )
                );

            if is_new_nickname_captured {
                return Err(Error::User(UserError::EditFailNicknameCaptured));
            }
        }

        let user = collection.get_mut(user_edit.id as usize)
            .and_then(|user| user.take())
            .ok_or(Error::User(UserError::UserDoesNotExists))?;

        let edited_user = User {
            name: user_edit.name.unwrap_or(user.name),
            nickname: user_edit.nickname.unwrap_or(user.nickname),
            password: user_edit.new_password.unwrap_or(user_edit.password),
            ..user
        };

        collection[edited_user.id as usize] = Some(edited_user.clone());

        Ok(edited_user)
    }

    pub async fn delete_user(&self, user_id: u32) -> Result<User> {
        let mut collection = self.users_collection.lock()
            .map_err(|_| Error::User(UserError::DeleteFail))?;

        let user = collection.get_mut(user_id as usize)
            .and_then(|user| user.take());

        user.ok_or(Error::User(UserError::UserDoesNotExists))
    }

    pub async fn authenticate(&self, user_id: &u32, password: &String) -> Result<()> {
        let collection = self.users_collection.lock()
            .map_err(|_| Error::User(UserError::AuthFail))?;

        let user = collection.get(*user_id as usize)
            .ok_or(Error::User(UserError::UserDoesNotExists))?
            .as_ref()
            .ok_or(Error::User(UserError::UserDoesNotExists))?;

        if user.password == *password {
            Ok(())
        } else {
            Err(Error::User(UserError::AuthFailInvalidParams))
        }
    }

    pub async fn login(&self, user_login: UserLogin) -> Result<User> {
        let collection = self.users_collection.lock()
            .map_err(|_| Error::User(UserError::LoginFail))?;

        let user = collection.iter()
            .find(|user_option| user_option.as_ref()
                .is_some_and(|user| user.nickname == user_login.nickname)
            )
            .ok_or(Error::User(UserError::UserDoesNotExists))?
            .as_ref()
            .ok_or(Error::User(UserError::UserDoesNotExists))?;

        if user.password == user_login.password {
            Ok(user.clone())
        } else {
            Err(Error::User(UserError::LoginFailInvalidParams))
        }
    }
}