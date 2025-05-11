use derive_more::{From, FromStr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, From, FromStr)]
#[serde(transparent)]
pub struct UserId(Uuid);
impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn value(&self) -> &Uuid {
        &self.0
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
            user_id: UserId::new(),
            handle_name,
            at_id,
            birthday,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn example() {
        // 新規作成
        let _user_id = UserId::new();

        // 既存の値から作成
        let _po = UserId::from(Uuid::now_v7());
        let _po = UserId::from(uuid::uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d456"));
        let _po = UserId::from_str("f47ac10b-58cc-4372-a567-0e02b2c3d456").unwrap();
        let _po = UserId::try_from(Uuid::now_v7()).unwrap();
    }
}
