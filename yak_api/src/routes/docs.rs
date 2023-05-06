use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use super::yak;
use crate::models;
#[derive(OpenApi)]
#[openapi(
        paths(
            yak::create_yak,
            yak::get_yaks,
            yak::delete_yak,
            yak::update_yak,
            yak::get_yak
        ),
        components(
            schemas(yak::YakCreate, yak::YakDelete, yak::YakUpdate,models::yak::Yak,  ),
        ),
        tags(
            (name = "yak", description = "Yak related operations")
        ),
         modifiers(&SecurityAddon)

    )]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        )
    }
}
