const MAX_HEADER_VALUE: usize = 3076;

pub fn split_headers(
    headers: &actix_web::http::header::HeaderMap
) -> 
    actix_web::http::header::HeaderMap 
{
    let mut output = headers.clone();

    if let Some(value) = headers.get("x-headers") {
        if value.len() > MAX_HEADER_VALUE {
            output.remove("x-headers");

            let mut split = 0;
            for chunk in value.as_bytes().chunks(MAX_HEADER_VALUE) {
                let id = split.to_string();
                let header_name = <actix_web::http::header::HeaderName as std::str::FromStr>::from_str(&format!("x-headers-{}", id)).unwrap();
                let mut header_value_vec = Vec::with_capacity(chunk.len() + 1);
                header_value_vec.extend_from_slice(chunk);
                let header_value = actix_web::http::header::HeaderValue::from_bytes(&header_value_vec).unwrap();
                output.insert(header_name, header_value);
                split += 1;
            }
        }
    }

    output
}