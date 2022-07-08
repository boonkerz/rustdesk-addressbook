use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Peer {
    pub id: String,
    pub uuid: String,
    pub user: String,
    pub hostname: String,
    pub alias: String,
    pub platform: String,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AbResponseData {
    pub tags: Vec<String>,
    pub peers: Vec<Peer>
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AbResponse {
    pub data: AbResponseData,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AbRequestData {
    pub tags: Vec<String>,
    pub peers: Vec<Peer>
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AbRequest {
    pub data: AbRequestData,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "peer")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub uuid: String,
    pub user: String,
    pub alias: String,
    pub hostname: String,
    pub platform: String,
    pub tags: String,
    pub created: DateTimeLocal
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
