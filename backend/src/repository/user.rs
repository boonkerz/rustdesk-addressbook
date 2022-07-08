use chrono::Local;
use common::user::{self, LoginResponse, User};
use common::user::{Entity as Users, LoginRequest, RegisterRequest, RegisterResponse};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};

use crate::service::hash::{hash_password, verify_password};
use crate::service::token::generate_token_response;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub db_conn: DatabaseConnection,
}

impl UserRepository {

    pub async fn current_user(&self, uuid: String) -> Result<LoginResponse, LoginResponse> {
        match Users::find()
        .filter(user::Column::Uuid.eq(uuid))
        .one(&self.db_conn).await.unwrap() {
            Some(user) => {
                let token = generate_token_response(&user).unwrap();

                Ok(LoginResponse {
                    user: User {
                        name: user.username,
                        uuid: user.uuid
                    },
                    access_token: token,
                    success: true,
                    ..Default::default()
                })
            },
            None => Err(LoginResponse {
                error: "Could not verify".to_string(),
                success: false,
                ..Default::default()
            })
        }
    }

    pub async fn login(&self, login_req: LoginRequest) -> Result<LoginResponse, LoginResponse> {
        let _exists = Users::find()
            .filter(user::Column::Username.eq(login_req.username.clone()))
            .one(&self.db_conn).await.unwrap();

        match _exists
        {
            Some(user) => {
                match verify_password(user.password.clone(), login_req.password) {
                    Ok(result) => {
                        if result {
                            
                            let token = generate_token_response(&user).unwrap();

                            Ok(LoginResponse {
                                user: User {
                                    name: user.username,
                                    uuid: user.uuid,
                                },
                                access_token: token,
                                success: true,
                                ..Default::default()
                            })
                        }else{
                            Err(LoginResponse {
                                error: "Could not verify".to_string(),
                                success: false,
                                ..Default::default()
                            })
                        }
                    },
                    Err(_) => 
                        Err(LoginResponse {
                            error: "Could not verify".to_string(),
                            success: false,
                            ..Default::default()
                        })
                    
                }
            }
            None => {
                Err(LoginResponse {
                    error: "Could not found".to_string(),
                    success: false,
                    ..Default::default()
                })
            }
        }
    }

    pub async fn register(&self, register_req: RegisterRequest) -> Result<RegisterResponse, RegisterResponse> {
        let _exist = Users::find()
            .filter(user::Column::Username.eq(register_req.username.clone()))
            .one(&self.db_conn).await.unwrap();

        match _exist {
            Some(_) => {
                Ok(RegisterResponse {
                    message: "This e-mail is using by some user, please enter another e-mail."
                        .to_string(),
                    success: false,
                })
            }
            None => {
                
                match hash_password(register_req.password.as_str()) {
                    Ok(password) => {
                        let user = user::ActiveModel {
                            uuid: Set(sea_orm::prelude::Uuid::new_v4().to_string()),
                            username: Set(register_req.username.clone()),
                            password: Set(password),
                            created: Set(Local::now()),
                            ..Default::default()
                        };
                        
                        // insert one
                        if user.insert(&self.db_conn).await.is_err() {
                            error!("failed to insert new user");
                        };
                        
                        Ok(RegisterResponse {
                            message: format!("User {} created", register_req.username),
                            success: true
                        })
                    }
                    Err(_) => {
                        Err(RegisterResponse {
                            message: "User can't created".to_string(),
                            success: true
                        })
                    }
                }
                
            }
        }
    }
}