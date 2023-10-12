use crate::model::ModelManager;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
}; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

pub fn page_test_route(mm: ModelManager) -> Router {
    Router::new()
        .route("/test", get(test_handler))
        .with_state(mm)
}

async fn test_handler(State(_mm): State<ModelManager>) -> impl IntoResponse {
    let template = HelloTemplate { name: "jane" };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
