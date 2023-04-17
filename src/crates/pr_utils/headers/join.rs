pub fn join(
    headers: &actix_web::http::header::HeaderMap
) -> 
    actix_web::http::header::HeaderMap 
{
    let mut return_val = actix_web::http::header::HeaderMap::new();
    let mut sorted = std::collections::BTreeMap::new();

    for (header, value) in headers.iter() {
        sorted.insert(header.as_str(), value.to_str().unwrap().to_string());
    }

    for (header, value) in sorted.iter() {
        let header_wo_index = header.strip_suffix(|c: char| c == '-' || c.is_ascii_digit())
            .unwrap_or(header);

        let header_name = actix_web::http::header::HeaderName::from_bytes(header_wo_index.as_bytes()).unwrap();

        let joined_value = format!("{}{}", match return_val.get(&header_name) {
            Some(header_value) => {
                header_value.to_str().unwrap_or_default()
            }
            None => {
                ""
            }
        }, value);

        return_val.insert(header_name, joined_value.parse().unwrap());
    }

    return_val
}

/*
(
    match crate::crates::pr_utils::headers::join::join(
        actix_web::HttpRequest::headers(&request)
    ).get(
        header_prefix.to_string() + 
        "headers-"
    ) 
    {
        Some(header_value) => {
            header_value.to_str().unwrap_or_default()
        }
        None => {
            "{}"
        }
    }
)
*/