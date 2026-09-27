use axum::{
    extract::OriginalUri,
    http::{Request, Uri},
};

fn try_normalize(uri: &Uri) -> Option<Uri> {
    let path = uri.path();
    if path == "/" {
        return None;
    }

    let trimmed = path.trim_end_matches('/');
    if trimmed.len() == path.len() {
        return None;
    }

    let mut builder = Uri::builder();
    if let Some(scheme) = uri.scheme() {
        builder = builder.scheme(scheme.clone());
    }
    if let Some(auth) = uri.authority() {
        builder = builder.authority(auth.clone());
    }

    let new_path_and_query = match uri.query() {
        Some(q) => {
            let mut s = String::with_capacity(trimmed.len() + 1 + q.len());
            s.push_str(trimmed);
            s.push('?');
            s.push_str(q);
            s
        }
        None => trimmed.to_string(),
    };

    builder
        .path_and_query(new_path_and_query)
        .build()
        .ok()
}

/// Strips a trailing `/` from the request path so that `/users/` is routed like
/// `/users`.
///
/// Must be applied around the whole `Router` via `ServiceBuilder::map_request`
/// (see `main.rs`), *not* via `Router::layer`: `Router::layer` runs after
/// routing has already happened, so rewriting the URI there is too late.
pub fn normalize_trailing_slash<B>(mut request: Request<B>) -> Request<B> {
    let original_uri = request.uri().clone();
    match try_normalize(&original_uri) {
        Some(new_uri) => {
            tracing::debug!(
                "Normalized trailing slash: `{}` -> `{}`",
                original_uri, new_uri
            );
            request.extensions_mut().insert(OriginalUri(original_uri));
            *request.uri_mut() = new_uri;
        }
        None => {
            tracing::trace!("No normalization needed for: `{}`", original_uri);
        }
    }
    request
}
