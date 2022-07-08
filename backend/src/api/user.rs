use actix_web::web::ReqData;
use actix_web::{web, HttpResponse, Result as ActixResult};
use common::user::{LoginRequest, RegisterRequest};

use crate::AppState;

pub async fn login(login_req: web::Json<LoginRequest>, state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    
    match state.user_repository.login(login_req.into_inner()).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Ok(HttpResponse::Ok().json(err))
    }
    
}

pub async fn register(register_req: web::Json<RegisterRequest>, state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    
    match state.user_repository.register(register_req.into_inner()).await {
        Ok(ok) => Ok(HttpResponse::Ok().json(ok)),
        Err(err) => Ok(HttpResponse::Ok().json(err))
    }
}

pub async fn current_user(user_uuid: Option<ReqData<String>>, state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    
    if let Some(user_uuid) = user_uuid {
        match state.user_repository.current_user(user_uuid.to_string()).await {
            Ok(result) => Ok(HttpResponse::Ok().json(result)),
            Err(err) => Ok(HttpResponse::Ok().json(err))
        }
    }else{
        Ok(HttpResponse::Forbidden().body("No User".to_string()))
    }
}