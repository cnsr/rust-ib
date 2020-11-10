mod route;
pub use route::*;

use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};

use actix_web::{Error,dev::ServiceRequest};

use crate::utils::validate_credentials;


pub async fn basic_auth_validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_credentials(credentials.password().unwrap().trim()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        },
        Err(_) => Err(AuthenticationError::from(config).into())
    }
}
