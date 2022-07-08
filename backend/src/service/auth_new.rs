use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_http::body::EitherBody;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::{future::LocalBoxFuture};
use log::{debug, error};

use crate::{api::routes, service::token::UserToken};

pub struct Auth;

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        println!("Hi from start. You requested: {}", req.path());

        // allow /auth/login and /auth/signup
        for ignore_route in routes::IGNORE_ROUTES.iter() {
            if req.path().starts_with(ignore_route) {
                debug!("The request path is in the ignored routes! It's a pass.");
                let res = self.service.call(req);
                return Box::pin(async move {
                    res.await.map(ServiceResponse::map_into_left_body)
                });
            }
        }

        debug!("Finding the authorization header...");
        let authen_header = match req.headers().get("Authorization") {
            Some(authen_header) => authen_header,
            None => {
                return Box::pin(async move {
                    let resp = HttpResponse::Unauthorized().finish().map_into_right_body();

                    Ok(req.into_response(resp))
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
        debug!("Raw Token {}", raw_token.to_string());
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
        

        Box::pin(async move {    
            let token_data = token;
            req.extensions_mut().insert(token_data.uuid);
            let res = svc.call(req);
            
            res.await.map(ServiceResponse::map_into_left_body)
            
        })
    }
}