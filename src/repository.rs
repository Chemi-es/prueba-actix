use crate::user::User;

pub trait Repository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}

pub struct MemoryRepository {
    users: Vec<User>,

}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new("Chemi".to_string(), (1970, 04, 23))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        self.users
        .iter()
        .find(|u| &u.id == user_id)
        .map(|u| u.clone())
        .ok_or_else(|| "UUID Invalido".to_string())
    }

}