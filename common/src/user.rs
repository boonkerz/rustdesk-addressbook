use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub id: String,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub success: bool,
    pub error: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct User {
    pub name: String,
    pub uuid: String,
}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub created: DateTimeLocal
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
