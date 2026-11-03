use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company_name: String,
    pub job_category: Option<String>,
    pub work_location: Option<String>,
    pub remote_note: Option<String>,
    pub salary_type: Option<String>,
    pub salary_min: Option<u64>,
    pub salary_max: Option<u64>,
    pub description: String,
    pub source_url: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub total_count: u64,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JobsResponse {
    pub data: Vec<Job>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JobDetailResponse {
    pub data: Job,
}

impl Job {
    /// 報酬の表示用文字列。例: "月額 800,000〜1,000,000円"
    pub fn salary_display(&self) -> String {
        let unit = self.salary_type.as_deref().unwrap_or("");
        let range = match (self.salary_min, self.salary_max) {
            (Some(min), Some(max)) if min > 0 && min == max => format!("{}円", group(min)),
            (Some(min), Some(max)) if min > 0 && max > 0 => {
                format!("{}〜{}円", group(min), group(max))
            }
            (Some(v), _) | (_, Some(v)) if v > 0 => format!("{}円", group(v)),
            _ => return "要相談".to_string(),
        };
        if unit.is_empty() {
            range
        } else {
            format!("{unit} {range}")
        }
    }

    /// 空白を畳んだ概要。`max_chars` を超える場合は末尾に "…" を付ける。
    pub fn summary(&self, max_chars: usize) -> String {
        let flat = self
            .description
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if flat.chars().count() <= max_chars {
            flat
        } else {
            let cut: String = flat.chars().take(max_chars).collect();
            format!("{cut}…")
        }
    }

    /// http(s) 以外のスキームは掲載元リンクとして使わない。
    pub fn safe_source_url(&self) -> Option<&str> {
        self.source_url
            .as_deref()
            .filter(|u| u.starts_with("https://") || u.starts_with("http://"))
    }
}

fn group(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(min: Option<u64>, max: Option<u64>, description: &str) -> Job {
        Job {
            id: "id1".into(),
            title: "t".into(),
            company_name: "c".into(),
            job_category: None,
            work_location: None,
            remote_note: None,
            salary_type: Some("月額".into()),
            salary_min: min,
            salary_max: max,
            description: description.into(),
            source_url: Some("https://example.com/1".into()),
            published_at: None,
        }
    }

    #[test]
    fn decodes_list_response_with_nulls_and_unknown_fields() {
        let json = include_str!("../../tests/fixtures/jobs.json");
        let res: JobsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.data.len(), 2);
        assert_eq!(res.data[0].company_name, "geechs 掲載案件");
        assert_eq!(res.data[0].remote_note, None);
        assert_eq!(res.pagination.total_count, 5893);
        assert!(res.pagination.next_cursor.is_some());
    }

    #[test]
    fn decodes_detail_response() {
        let json = include_str!("../../tests/fixtures/job_detail.json");
        let res: JobDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res.data.id, "cmvgblyxf18jnkyv3eqke6i0w");
    }

    #[test]
    fn salary_range_and_fixed() {
        assert_eq!(
            job(Some(800000), Some(1000000), "").salary_display(),
            "月額 800,000〜1,000,000円"
        );
        assert_eq!(
            job(Some(900000), Some(900000), "").salary_display(),
            "月額 900,000円"
        );
        assert_eq!(job(Some(0), Some(0), "").salary_display(), "要相談");
        assert_eq!(job(None, None, "").salary_display(), "要相談");
    }

    #[test]
    fn summary_collapses_whitespace_and_truncates() {
        assert_eq!(job(None, None, "a\n\n b").summary(10), "a b");
        assert_eq!(job(None, None, "あいうえお").summary(3), "あいう…");
    }

    #[test]
    fn rejects_non_http_source_url() {
        let mut j = job(None, None, "");
        assert!(j.safe_source_url().is_some());
        j.source_url = Some("javascript:alert(1)".into());
        assert!(j.safe_source_url().is_none());
    }
}
