use chrono::Local;
use common::tag::{Entity as Tags, self, Model as TagEntity};
use common::peer::{Entity as Peers, self, Model as PeerEntity, Peer};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set};

#[derive(Debug, Clone)]
pub struct AbRepository {
    pub db_conn: DatabaseConnection,
}

impl AbRepository {

    pub async fn find_tags_by_user_uuid(&self, uuid: String) -> Vec<TagEntity> {
        let tags = Tags::find()
        .filter(tag::Column::User.eq(uuid))
        .all(&self.db_conn).await.unwrap();

        return tags;
    }

    pub async fn update_tags_by_user_uuid(&self, uuid: String, tags: &Vec<String>) {
        if Tags::delete_many()
        .filter(tag::Column::User.eq(uuid.to_owned()))
        .exec(&self.db_conn).await.is_err() {
            error!("delete Tags goes wrong");
        }

        for ele in tags {
            let tag = tag::ActiveModel {
                name: Set(ele.to_owned()),
                user: Set(uuid.to_owned()),
                ..Default::default() // all other attributes are `NotSet`
            };
            
            if tag::Entity::insert(tag).exec(&self.db_conn).await.is_err() {
                error!("insert Tags goes wrong");
            }
        }

        
    }

    pub async fn update_peers_by_user_uuid(&self, uuid: String, peers: &Vec<Peer>) {
        if Peers::delete_many()
        .filter(peer::Column::User.eq(uuid.to_owned()))
        .exec(&self.db_conn).await.is_err() {
            error!("delete peers goes wrong");
        }

        for ele in peers {
            let peer = peer::ActiveModel {
                alias: Set(ele.alias.to_owned()),
                id: Set(ele.id.to_owned()),
                hostname: Set(ele.hostname.to_owned()),
                platform: Set(ele.platform.to_owned()),
                tags: Set(ele.tags.join(";").to_string()),
                user: Set(uuid.to_owned()),
                created: Set(Local::now()),
                uuid: Set(sea_orm::prelude::Uuid::new_v4().to_string()),
                ..Default::default() // all other attributes are `NotSet`
            };
            
            if peer::Entity::insert(peer).exec(&self.db_conn).await.is_err() {
                error!("insert Peer goes wrong");
            }
        }

        
    }

    pub async fn find_peers_by_user_uuid(&self, uuid: String) -> Vec<PeerEntity> {
        let peers = Peers::find()
        .filter(peer::Column::User.eq(uuid))
        .all(&self.db_conn).await.unwrap();

        return peers;
    }
    
}