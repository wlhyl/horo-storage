use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};

use crate::handlers::{
    native::{
        __path_delete_native, __path_get_native_pages, __path_insert_native, __path_update_native,
    },
    user::{__path_login, __path_update_user},
};

use crate::request::{GeoRequest, LoginUserRequest, NativeRequest, UpdateUserRequest};
use crate::response::{Geo, Native, Token};

#[derive(OpenApi)]
#[openapi(
    paths(
    
    // native
    get_native_pages, insert_native,
    update_native,
     delete_native,
    // user
        login, update_user,
    ),
    components(schemas(
         GeoRequest,
    // native
        Native, Geo, NativeRequest,
    // user
        LoginUserRequest, UpdateUserRequest, Token,
    )),
    tags(
        (name = "storage API", description = "API")
    ),
// security(("token" = []))
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut default_components = Components::default();
        let components = openapi
            .components
            .as_mut()
            .unwrap_or(&mut default_components);
        let value = ApiKeyValue::with_description("token", "认证token");
        let scheme = SecurityScheme::ApiKey(ApiKey::Header(value));
        components.add_security_scheme("token", scheme);
    }
}