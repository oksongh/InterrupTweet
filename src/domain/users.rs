use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UserId(Uuid);
impl UserId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: UserId,
    pub handle_name: String,
    pub at_id: String,
    pub birthday: Option<chrono::NaiveDate>,
}

impl User {
    pub fn new(
        user_id: UserId,
        handle_name: String,
        at_id: String,
        birthday: Option<chrono::NaiveDate>,
    ) -> Self {
        Self {
            user_id,
            handle_name,
            at_id,
            birthday,
        }
    }

    pub fn generate_new_user(
        handle_name: String,
        at_id: String,
        birthday: Option<chrono::NaiveDate>,
    ) -> Self {
        Self {
            user_id: UserId(Uuid::now_v7()),
            handle_name,
            at_id,
            birthday,
        }
    }
}
