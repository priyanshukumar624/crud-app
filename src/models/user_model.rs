use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}
