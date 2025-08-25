use axum::{extract::FromRequestParts, response::IntoResponse};
use http::{header::FORWARDED, request::Parts, HeaderMap, StatusCode};

const X_FORWARDED_HOST_HEADER_KEY: &str = "X-Forwarded-Host";

pub struct Host(pub String);

impl<S> FromRequestParts<S> for Host
where
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = HostRejection;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(host) = parse_forwarded(&parts.headers) {
            return Ok(Host(host.to_owned()));
        }

        if let Some(host) = parts
            .headers
            .get(X_FORWARDED_HOST_HEADER_KEY)
            .and_then(|host| host.to_str().ok())
        {
            return Ok(Host(host.to_owned()));
        }

        if let Some(host) = parts
            .headers
            .get(http::header::HOST)
            .and_then(|host| host.to_str().ok())
        {
            return Ok(Host(host.to_owned()));
        }

        if let Some(host) = parts.uri.host() {
            return Ok(Host(host.to_owned()));
        }

        Err(HostRejection)
    }
}

fn parse_forwarded(headers: &HeaderMap) -> Option<&str> {
    // if there are multiple `Forwarded` `HeaderMap::get` will return the first one
    let forwarded_values = headers.get(FORWARDED)?.to_str().ok()?;

    // get the first set of values
    let first_value = forwarded_values.split(',').next()?;

    // find the value of the `host` field
    first_value.split(';').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        key.trim()
            .eq_ignore_ascii_case("host")
            .then(|| value.trim().trim_matches('"'))
    })
}

pub struct HostRejection;

impl IntoResponse for HostRejection {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}
