// github.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub open_issues_count: u32,
    pub updated_at: String,
    pub created_at: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Issue {
    pub title: String,
    pub state: String,
    pub body: Option<String>,
    pub number: u32,
    pub html_url: String,
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PR {
    pub number: u32,
    pub state: String,
    pub body: Option<String>,
    pub title: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Label {
    pub url: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub name: String,
}
