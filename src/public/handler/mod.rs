// mod health;
// mod introspect;
// mod trace;

// use url::Url;

// pub(crate) use health::*;
// pub(crate) use introspect::handler_introspect;
// pub(crate) use trace::handler_trace;

use axum::http::StatusCode;
use axum::response::{
  Html,
  IntoResponse,
};

pub async fn index() -> Html<&'static str> {
  Html("<h1>Welcome to Haya</h1>")
}

pub async fn not_found() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Not found")
}

// pub(crate) async fn error_handler(err: routerify::RouteError) -> Response<Body> {
//     let svc_err = err.downcast::<ApiError>().unwrap();

//     match svc_err.as_ref() {
//         ApiError::Http(e) => {
//             let data = serde_json::json!({
//                 "success": false,
//                 "message": e.to_string(),
//             });

//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
//                 .body(Body::from(data.to_string()))
//                 .map_err(ServiceError::Http)
//                 .unwrap()
//         }
//         ApiError::Authorize(e) => {
//             let mut url = Url::parse(e.redirect_uri.as_str()).unwrap();
//             url.query_pairs_mut()
//                 .clear()
//                 .append_pair("error", e.error.to_string().as_str())
//                 .append_pair("error_description", e.error_description.as_str())
//                 .append_pair("state", e.state.as_str());

//             Response::builder()
//                 .status(StatusCode::FOUND)
//                 .header(HeaderValues::LOCATION, url.as_str())
//                 .body(Body::empty())
//                 .map_err(ServiceError::Http)
//                 .unwrap()
//         }
//         ApiError::Token(e) => {
//             let data = serde_json::json!({
//                 "error": e.error.to_string(),
//                 "error_description": e.error_description.as_str(),
//                 "state": e.state.as_str(),
//             });

//             Response::builder()
//                 .status(StatusCode::BAD_REQUEST)
//                 .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
//                 .body(Body::from(data.to_string()))
//                 .map_err(ServiceError::Http)
//                 .unwrap()
//         }
//         _ => {
//             log::error!("Error trace {:?}", svc_err);

//             let data = serde_json::json!({
//                 "success": false,
//                 "message": svc_err.to_string(),
//             });

//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
//                 .body(Body::from(data.to_string()))
//                 .map_err(ServiceError::Http)
//                 .unwrap()
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_handler_index_should_ok() {
//         let req = Request::<Body>::default();
//         let resp = handler_index(req).await.unwrap();
//         assert_eq!(resp.status(), StatusCode::OK);
//     }

//     #[tokio::test]
//     async fn test_handler_404_should_ok() {
//         let data = serde_json::json!({
//             "success": false,
//             "message": "Route not found",
//         });

//         let req = Request::<Body>::default();
//         let resp = handler_not_found(req).await.unwrap();
//         let (parts, body) = resp.into_parts();

//         let body_raw = hyper::body::to_bytes(body).await.unwrap();
//         let body = String::from_utf8(body_raw.to_vec()).unwrap();
//         assert_eq!(parts.status, StatusCode::NOT_FOUND);
//         assert_eq!(data.to_string(), body);
//     }
// }
