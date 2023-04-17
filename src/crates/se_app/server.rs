use crate::config;
pub struct Server;

impl Server {
    pub async fn initialize(

    ) ->    
        std::io::Result<
            ()
        > 
    {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        let mut builder = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
        builder.init();

        let bind_ip = config().se_app.bind_ip.clone();
        let bind_port = config().se_app.port.clone();

        println!("Starting server at {}:{}", bind_ip, bind_port);

/*
        let mut builder =
            openssl::ssl::SslAcceptor::mozilla_intermediate(
                openssl::ssl::SslMethod::tls()
            )
            .unwrap();
        builder
            .set_private_key_file(
                config().se_app.ssl.key.to_owned(),
                openssl::ssl::SslFiletype::PEM,
            )
            .unwrap();
        builder
            .set_certificate_chain_file(
                config().se_app.ssl.cert.to_owned()
            )
            .unwrap();
*/
        actix_web::HttpServer::new(
            super::configure::create_app
        )
            .client_request_timeout(
                std::time::Duration::from_millis(2500)
            ) 

            .workers(
                config().se_app.threads
            )

            /*.bind_openssl(
                (bind_ip, bind_port), 
                builder
            )?*/

            .max_connections(
                50_000
            )

            .bind(
                (bind_ip, bind_port)
            ).unwrap()

            .run()
            .await
    }
}
