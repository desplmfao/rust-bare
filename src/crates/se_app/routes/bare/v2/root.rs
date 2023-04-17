use futures::stream::StreamExt;

pub async fn root(
    client: actix_web::web::Data<awc::Client>,
    request: actix_web::HttpRequest,
    mut payload: actix_web::web::Payload,
) -> 
    actix_web::HttpResponse
{
    let header_prefix = &crate::config().se_app.header_prefix;

    let req_proto = match actix_web::HttpRequest::headers(&request).get(
        header_prefix.to_string() + 
        "protocol"
    ) {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            ""
        }
    };

    let req_host = match actix_web::HttpRequest::headers(&request).get(
        header_prefix.to_string() + 
        "host"
    ) {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            ""
        }
    };

    let req_port = match actix_web::HttpRequest::headers(&request).get(
        header_prefix.to_string() + 
        "port"
    ) {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            ""
        }
    };

    let req_path = match actix_web::HttpRequest::headers(&request).get(
        header_prefix.to_string() + 
        "path"
    ) {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            ""
        }
    };

    let req_url = req_proto.to_owned() + "//" + req_host + ":" + req_port + req_path;

    let headers_string = match crate::crates::pr_utils::headers::join::join(
        actix_web::HttpRequest::headers(&request)
    ).get(
        header_prefix.to_string() + "headers"
    ) {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            "{}"
        }
    }.to_string();
    
    println!("{:?}", req_url);

    let headers_json: serde_json::Value = match serde_json::from_str(&headers_string) {
        Ok(v) => v,
        Err(e) => {
            return actix_web::HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "type": "header",
                    "error": e.to_string(),
                })
            );
        }
    };

    //println!("{:?}", headers_string);
    //println!("{:#?}", headers_json);

    let mut bytes = actix_web::web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        bytes.extend_from_slice(&chunk.unwrap());
    };

    let mut request_builder = client.request(
        actix_web::HttpRequest::method(&request).into(),
        req_url.to_owned(),
    );

    if let serde_json::Value::Object(headers_map) = headers_json {
        for (key, value) in headers_map {
            if let serde_json::Value::String(header_value) = value {
                request_builder = request_builder.append_header((key, header_value));
            }
        }
    }

    let mut response = match request_builder
        .no_decompress()
        .send_body(bytes)
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return actix_web::HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "type": "request",
                    "error": e.to_string(),
                })
            );
        }
    };

    let mut headers_map = serde_json::Map::new();

    for (name, value) in response.headers() {
        match value.to_str() {
            Ok(value_str) => {
                headers_map.insert(name.as_str().to_owned(), serde_json::json!(value_str));
            },
            Err(e) => {
                eprintln!("Error converting header value to string: {}", e);
            }
        }
    }

    let headers_json_ = serde_json::json!(headers_map);
    let headers_string = headers_json_.to_string();

    println!("{:#?}", headers_string);

    return actix_web::HttpResponse::Ok()
        //.status(actix_web::http::StatusCode::PROCESSING)
        .append_header((
            header_prefix.to_string() + "status", 
                response.status().as_str()
        ))
        .append_header((
            header_prefix.to_string() + "status-message", 
                response.status().to_string().split_at(3).1
        ))
        .append_header((
            header_prefix.to_string() + "headers", 
                headers_string
        ))
        .body(
            match response
                .body()
                .limit(crate::config().se_app.size_limit)
                .await
            {
                Ok(resp) => resp.to_vec(),
                Err(e) => {
                    return actix_web::HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "type": "response",
                            "error": e.to_string(),
                        })
                    );
                }
            },
        );
}
