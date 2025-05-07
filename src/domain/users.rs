use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: i64,
    pub handle_name: String,
    pub at_id: String,
    pub birthday: Option<String>,
}

impl User {
    pub fn new(user_id: i64, handle_name: String, at_id: String, birthday: Option<String>) -> Self {
        Self {
            user_id,
            handle_name,
            at_id,
            birthday,
        }
    }

    pub fn new_without_birthday(user_id: i64, handle_name: String, at_id: String) -> Self {
        Self {
            user_id,
            handle_name,
            at_id,
            birthday: None,
        }
    }
}
