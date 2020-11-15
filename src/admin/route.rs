use crate::admin::*;

use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder, Error as ActixError};
use sqlx::{Postgres, Pool};
use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};

use crate::utils::validate_credentials;
use crate::middlewares::auth::create_jwt;

// ADMIN apps in the 'admin' scope don't need to have full path specified


#[derive(Deserialize, Clone)]
struct AdminLoginRequest {
    password: String
}

#[derive(Serialize)]
struct AdminJWTResponse {
    jwt_token: String
}

impl Responder for AdminJWTResponse {
    type Error = ActixError;
    type Future = Ready<Result<HttpResponse, ActixError>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

//unprotected route
#[post("/login/")]
async fn admin_login(credentials: web::Json<AdminLoginRequest>) -> impl Responder {
    match validate_credentials(credentials.password.as_str()) {
        Ok(validation) => {
            if validation {
                let token_result = create_jwt();
                match token_result {
                    Ok(token) => {
                        return HttpResponse::Ok().json(AdminJWTResponse {jwt_token: token});
                    },
                    Err(error) => {
                        return HttpResponse::BadRequest().body(error.to_string());
                    }
                }
            } else {
                return HttpResponse::BadRequest().body("Password does not match.");
            }
        },
        Err(_) => HttpResponse::BadRequest().body("Bad request.")
    }
}

#[post("/verify/")]
async fn admin_verify() -> impl Responder {
    HttpResponse::Ok()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(admin_verify);
}

pub fn init_unprotected_routes(config: &mut web::ServiceConfig) {
    config.service(admin_login);
}