use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{extract::Form, response::Html};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Deserialize, Debug)]
pub struct Input {
    pub query: String,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct MainFormTemplate;

// struct HtmlTemplate<T>(T);
//
// impl<T> IntoResponse for HtmlTemplate<T>
// where
//     T: Template,
// {
//     fn into_response(self) -> Response {
//         match self.0.render() {
//             Ok(html) => Html(html).into_response(),
//             Err(err) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Failed to render template. Error: {err}"),
//             )
//                 .into_response(),
//         }
//     }
// }
impl IntoResponse for MainFormTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Clone, FromRow, Debug, Template)]
#[template(path = "accept_form_pawned.html")]
pub struct AcceptFormPawned {
    email: String,
}

impl IntoResponse for AcceptFormPawned {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Clone, FromRow, Debug, Template)]
#[template(path = "accept_form_secure.html")]
pub struct AcceptFormSecure {
    email: String,
}

impl IntoResponse for AcceptFormSecure {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

pub async fn show_form() -> impl IntoResponse {
    let form = MainFormTemplate;
    form
}

#[allow(dead_code)]
enum AcceptFormResponse {
    Pawned(AcceptFormPawned),
    Secure(AcceptFormSecure),
}

impl IntoResponse for AcceptFormResponse {
    fn into_response(self) -> Response {
        match self {
            AcceptFormResponse::Pawned(response) => response.into_response(),
            AcceptFormResponse::Secure(response) => response.into_response(),
        }
    }
}

pub async fn _accept_form_with_enums(
    State(pool): State<SqlitePool>,
    Form(input): Form<Input>,
    // took me an hour this error
) -> impl IntoResponse {
    // Check if the input email exists in the database
    let found = sqlx::query_as::<_, AcceptFormPawned>(
        "SELECT email FROM fishy_website_com WHERE email = ?",
    )
    .bind(&input.query)
    // .fetch_one(&pool) .await;
    .fetch_optional(&pool)
    .await
    .unwrap();

    let response = match found {
        Some(email) => {
            println!("found email");
            AcceptFormResponse::Pawned(email)
        }
        None => {
            println!("email not found");
            AcceptFormResponse::Secure(AcceptFormSecure { email: input.query })
        }
    };

    response
}

#[derive(Debug, Serialize)]
struct CheckResult {
    email: String,
    found: bool,
}

impl IntoResponse for CheckResult {
    fn into_response(self) -> Response {
        Json::into_response(Json(self))
    }
}

pub async fn accept_form(
    State(pool): State<SqlitePool>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    // Check if the input email exists in the database
    let found = sqlx::query_as::<_, AcceptFormPawned>(
        "SELECT email FROM fishy_website_com WHERE email = ?",
    )
    .bind(&input.query)
    .fetch_optional(&pool)
    .await
    .unwrap();

    match found {
        Some(email) => {
            println!("{} found", email.email);
            Json(CheckResult {
                email: email.email.to_string(),
                found: true,
            })
        }
        None => {
            println!("{} not found", input.query);
            Json(CheckResult {
                email: input.query.to_string(),
                found: false,
            })
        }
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
