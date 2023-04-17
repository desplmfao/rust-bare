pub async fn root(

) -> 
    actix_web::HttpResponse 
{
    let config = &crate::config();
    let mut process_psutil = psutil::process::Process::new(std::process::id()).unwrap();

    actix_web::HttpResponse::Ok()
        .json(
            serde_json::json!({
                "versions": [ "v2" ],
                "memoryUsage": (process_psutil.memory_info().unwrap().rss()).to_string() + "b",
                "maintainer": {

                },
                "project": {
                    "name": "",
                    "description": "",
                    "email": "",
                    "website": "",
                    "repository": "",
                    "version": "0"
                },
                "language": "Rust",
                "misc": {
                    "sizeLimit": config.se_app.size_limit,
                }
            })
        )
}
