use crate::config;

use actix_web::dev::*;
use actix_web::*;
use actix_web::web::*;

use actix_files::Files;

pub fn create_app(

) -> 
    App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<
                impl body::MessageBody
            >,
            Config = (),
            InitError = (),
            Error = Error,
        >,
    > 
{
    let max_payload_size = config().se_app.size_limit;

    let client = awc::ClientBuilder::new()
        .max_http_version(awc::http::Version::HTTP_2)
        .disable_redirects()
        .no_default_headers()
        .max_redirects(0)
        .timeout(std::time::Duration::from_millis(5000))
        .finish();

    // wrap the client in a mutex and wrap the mutex in web::Data
    let client_data = Data::new(client);

    actix_web::App::new()
        .configure(s_configure)
        .app_data(PayloadConfig::new(max_payload_size.try_into().unwrap()))
        .app_data(client_data.clone())
        .service(
            actix_web::web::resource("/bare")
                .to(super::routes::bare::root::root)
        )
        .service(
            actix_web::web::resource("/bare/")
                .to(super::routes::bare::root::root)
        )
        .service(
            actix_web::web::resource("/bare/v2")
                .to(super::routes::bare::v2::root::root)
        )
        .service(
            actix_web::web::resource("/bare/v2/")
                .to(super::routes::bare::v2::root::root)
        )
        /*.service(
            actix_web::web::resource("/bare/v2/ws-meta")
                .to(super::routes::api::v2::ws_meta::root::root)
        )*/
        /*.service(
            actix_web::web::resource("/bare/v2/ws-meta/")
                .to(super::routes::api::v2::ws_meta::root::root)
        )*/
        .service(
            Files::new("/", &config().se_app.public)
                .prefer_utf8(true)
        )
}

pub fn s_configure(
    _cfg: &mut actix_web::web::ServiceConfig
) {

}