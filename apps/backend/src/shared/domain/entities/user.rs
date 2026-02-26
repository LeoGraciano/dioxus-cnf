use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub profile: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserProfile {
    Collector,
    Manager,
    Admin,
    Support,
}
