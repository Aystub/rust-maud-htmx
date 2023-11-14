use maud::{DOCTYPE, html, Markup};
use axum::{extract, http::StatusCode, Json, response::IntoResponse, routing::get, routing::post, Router};
use tower_http::{
    services::{ServeDir},
    trace::TraceLayer,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FormData {
    name: String,
}

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            script src="static/js/htmx.v1.9.8.min.js" {}
            script src="static/js/htmx.v1.9.8.json-enc.js" {}
            link rel="stylesheet" href="static/css/index.css" {}
        }
        img src="static/images/doodle.png";

        #content {
            p { "Hello world!" }
        }

        div {
            label for="name-input" {
                "Name"
            }
            input type="text" id="name" name="name";
            button hx-post="/html" hx-include="[name='name']" hx-ext="json-enc" hx-target="#content" hx-swap="outerHTML" {
                "Submit!"
            }
        }
    }
}

async fn json_response(extract::Json(payload): extract::Json<FormData>) -> Json<FormData> {
    tracing::debug!("click! name:: {}", payload.name);
    Json(payload)
}

async fn html_response(extract::Json(payload): extract::Json<FormData>) -> impl IntoResponse {
    (StatusCode::OK, html! {
        #content {
            p { "Hello " (payload.name) "! This is HTMX." }
        }
    })
}

#[tokio::main]
async fn main() {
    // enable tracing
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/json", post(json_response))
        .route("/html", post(html_response))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}