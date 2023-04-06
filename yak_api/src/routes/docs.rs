use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use super::yak;

#[derive(OpenApi)]
#[openapi(
        paths(
            yak::create_yak,
        ),
        components(
            schemas(yak::YakCreate)
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
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
