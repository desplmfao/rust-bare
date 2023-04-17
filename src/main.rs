pub mod crates;

static CONFIG: once_cell::sync::OnceCell<crate::crates::config::Config> = once_cell::sync::OnceCell::new();

#[actix_web::main]
async fn main() {
    let config = crate::crates::config::Config::load("./global_config.json");

    CONFIG.set(
        config.await
    ).unwrap();

    crate::crates::se_app::server::Server::initialize(

    )
        .await
        .expect("ERR(server init)");
}

fn config() -> 
    &'static crate::crates::config::Config 
{
    CONFIG.get().unwrap()
}