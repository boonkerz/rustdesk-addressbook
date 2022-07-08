use std::{future::{ready, Ready}};

use actix_http::body::{BoxBody, EitherBody};
use actix_rt::task::spawn_blocking;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use common::user::Model;
use futures_util::{future::LocalBoxFuture, Future};

use log::{ error, debug};
use crate::{api::routes, service::token::UserToken, AppState};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // allow /auth/login and /auth/signup
        for ignore_route in routes::IGNORE_ROUTES.iter() {
            if req.path().starts_with(ignore_route) {
                debug!("The request path is in the ignored routes! It's a pass.");
                let future = self.service.call(req);
                return Box::pin(async move {
                    let response = future.await?.map_into_left_body();
                    Ok(response)
                });
            }
        }

        debug!("Finding the authorization header...");
        let authen_header = match req.headers().get("Authorization") {
            Some(authen_header) => authen_header,
            None => {
                return Box::pin(async move {
                    let resp = HttpResponse::Unauthorized()
                    .body("We did not find an authentication header.");

                    Ok(req.into_response(resp).map_into_right_body())
                })
            }
        };

        debug!("Parsing authorization header...");
        let str_authen_header = match authen_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .body(
                                "The authorization header doesn't seem to be stringifyable"
                            ),
                    ).map_into_right_body())
                });
            }
        };

        debug!(
            "Checking the start of the authorization header: {}",
            str_authen_header
        );
        if !str_authen_header.starts_with("Bearer")
            && !str_authen_header.starts_with("bearer")
        {
            return Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body("The authorization header doesn't start with 'bearer'"),
                ).map_into_right_body())
            });
        }

        debug!("Parsing token");
        let raw_token = str_authen_header[6..str_authen_header.len()].trim();

        debug!("Decoding the token");
        let token = match UserToken::decode_from_string(raw_token.to_string()) {
            Ok(decoded_data) => decoded_data,
            Err(decode_error) => {
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .body(format!("Could not decode the token: {}", decode_error))
                            .map_into_right_body(),
                    ))
                });
            }
        };

        let appData = req.app_data::<AppState>().unwrap();
        if !UserToken::is_still_valid(&token) {
            error!("invalid jwt");
            return Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body(format!("Invalid Token"))
                        .map_into_right_body(),
                ))
            });
        }
       

        

        let svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            println!("request body: {body:?}");
            let res = svc.call(req).await?;

            println!("response: {:?}", res.headers());
            Ok(res)
        })
/*
        async {
            if let Err(user) = appData.user_repository.find_user_by_uuid(token.uuid).await {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body(format!("Token Error"))
                        .map_into_right_body(),
                )) 
            }else{
                Ok()  
            }
        };
        

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?.map_into_left_body();

            println!("Hi from response");
            Ok(res)
        })*/
    }
}