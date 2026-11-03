use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

use crate::AppState;
use crate::api::ApiError;
use crate::models::Job;

const PAGE_SIZE: u32 = 20;
const SUMMARY_CHARS: usize = 150;

pub struct JobView {
    pub id: String,
    pub title: String,
    pub company_name: String,
    pub salary: String,
    pub summary: String,
    pub source_url: Option<String>,
}

impl From<&Job> for JobView {
    fn from(job: &Job) -> Self {
        Self {
            id: job.id.clone(),
            title: job.title.clone(),
            company_name: job.company_name.clone(),
            salary: job.salary_display(),
            summary: job.summary(SUMMARY_CHARS),
            source_url: job.safe_source_url().map(str::to_string),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub jobs: Vec<JobView>,
    pub total_count: u64,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub message: &'static str,
}

pub async fn index(State(state): State<AppState>) -> Response {
    match state.client.list_jobs(PAGE_SIZE, None, None).await {
        Ok(res) => render(
            StatusCode::OK,
            IndexTemplate {
                jobs: res.data.iter().map(JobView::from).collect(),
                total_count: res.pagination.total_count,
            },
        ),
        Err(e) => error_page(&e),
    }
}

fn error_page(err: &ApiError) -> Response {
    eprintln!("jobs api error: {err}");
    let (status, message) = match err {
        ApiError::Timeout => (
            StatusCode::GATEWAY_TIMEOUT,
            "案件情報の取得がタイムアウトしました。しばらくしてから再度お試しください。",
        ),
        _ => (
            StatusCode::BAD_GATEWAY,
            "案件情報を取得できませんでした。しばらくしてから再度お試しください。",
        ),
    };
    render(status, ErrorTemplate { message })
}

fn render<T: Template>(status: StatusCode, template: T) -> Response {
    match template.render() {
        Ok(body) => (status, Html(body)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response(),
    }
}
