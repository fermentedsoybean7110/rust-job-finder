use std::fmt;
use std::time::Duration;

use reqwest::StatusCode;

use crate::models::{Job, JobDetailResponse, JobsResponse};

const TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Timeout,
    Network,
    Status(u16),
    Decode,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::Timeout => write!(f, "request timed out"),
            Self::Network => write!(f, "network error"),
            Self::Status(code) => write!(f, "unexpected status {code}"),
            Self::Decode => write!(f, "failed to decode response"),
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Clone)]
pub struct JobsClient {
    http: reqwest::Client,
    base_url: String,
}

impl JobsClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub async fn list_jobs(
        &self,
        limit: u32,
        category: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<JobsResponse, ApiError> {
        let mut query = vec![("limit", limit.to_string())];
        if let Some(c) = category {
            query.push(("category", c.to_string()));
        }
        if let Some(c) = cursor {
            query.push(("cursor", c.to_string()));
        }
        self.get(&format!("{}/jobs", self.base_url), &query).await
    }

    pub async fn get_job(&self, id: &str) -> Result<Job, ApiError> {
        let url = format!("{}/jobs/{}", self.base_url, encode_path_segment(id));
        let res: JobDetailResponse = self.get(&url, &[]).await?;
        Ok(res.data)
    }

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        query: &[(&str, String)],
    ) -> Result<T, ApiError> {
        let res = self.http.get(url).query(query).send().await.map_err(|e| {
            if e.is_timeout() {
                ApiError::Timeout
            } else {
                ApiError::Network
            }
        })?;
        match res.status() {
            s if s.is_success() => res.json::<T>().await.map_err(|e| {
                if e.is_timeout() {
                    ApiError::Timeout
                } else if e.is_decode() {
                    ApiError::Decode
                } else {
                    ApiError::Network
                }
            }),
            StatusCode::NOT_FOUND => Err(ApiError::NotFound),
            s => Err(ApiError::Status(s.as_u16())),
        }
    }
}

fn encode_path_segment(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::encode_path_segment;

    #[test]
    fn encodes_path_segment() {
        assert_eq!(encode_path_segment("abc-1_2"), "abc-1_2");
        assert_eq!(encode_path_segment("a/b ?"), "a%2Fb%20%3F");
    }
}
