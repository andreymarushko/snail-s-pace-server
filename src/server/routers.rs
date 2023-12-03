use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use http::StatusCode;
use sailfish::TemplateOnce;

pub fn router_pages() -> Router {
    Router::new()
        .route("/", get(homepage))
        .route("/games", get(|| async { "Hello game list" }))
        .route("/tutorials", get(|| async { "Hello tutorials" }))
        .route("/blog", get(|| async { "Hello blog posts" }))
        .route("/game_frame/", get(|| async { "Here be iframe" }))
}

pub fn router_games() -> Router {
    Router::new().route("/", get(|| async { "Hello games" }))
}

#[derive(TemplateOnce)]
#[template(path = "header.html")]
struct Header {
    title: String,
}

#[derive(TemplateOnce)]
#[template(path = "home.html")]
struct Home {
    header: Header,
}

async fn homepage() -> impl IntoResponse {
    let templ = Home {
        header: Header {
            title: "Home".to_owned(),
        },
    };
    render(templ)
}

fn render(templ: impl TemplateOnce) -> Response {
    match templ.render_once() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {err}"),
        )
            .into_response(),
    }
}

// impl<T> IntoResponse for T
// where
//     T: TemplateOnce,
// {
//     fn into_response(self) -> Response {
//         match self.render() {
//             Ok(html) => Html(html).into_response(),
//             Err(err) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Failed to render template. Error: {err}"),
//             )
//                 .into_response(),
//         }
//     }
// }
