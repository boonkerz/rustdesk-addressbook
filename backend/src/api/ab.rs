use actix_web::{web::{self, ReqData}, Result as ActixResult, HttpResponse};
use common::peer::{Peer, AbResponse, AbRequest, AbResponseData};
use crate::{AppState};

pub async fn get(user_uuid: Option<ReqData<String>>, state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    if let Some(user_uuid) = user_uuid {
    
        let tags = state.ab_repository.find_tags_by_user_uuid(user_uuid.to_string()).
                    await.into_iter().map(|b| b.name).collect::<Vec<String>>();

        let peers = state.ab_repository.find_peers_by_user_uuid(user_uuid.to_string()).
                    await.into_iter().map(|b| Peer {
                        id: b.id.to_string(),
                        hostname: b.hostname,
                        platform: b.platform,
                        alias: b.alias,
                        tags: b.tags.split(";").map(|b|b.to_string()).collect(),
                        ..Default::default()
                    }).collect::<Vec<Peer>>();
            
        Ok(HttpResponse::Ok().json(
            AbResponse {
                data: AbResponseData {
                    peers: peers,
                    tags: tags
                },
                updated_at: "2022-12-12 12:12:12".to_string()
            }
        ))
    }else{
        Ok(HttpResponse::Forbidden().body("No User".to_string()))
    }
}

pub async fn post(ab_req: web::Json<AbRequest>, user_uuid: Option<ReqData<String>>, state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    if let Some(user_uuid) = user_uuid {
        
        state.ab_repository.update_tags_by_user_uuid(user_uuid.to_string(), &ab_req.data.tags).await;
        state.ab_repository.update_peers_by_user_uuid(user_uuid.to_string(), &ab_req.data.peers).await;

        Ok(HttpResponse::Ok().finish())
    }else{
        Ok(HttpResponse::Forbidden().body("No User".to_string()))
    }
}