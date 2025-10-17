use crate::models::{Issue, PR, Repo};
use reqwest::Client;
use serde::Deserialize;

// Response structure for search API
#[derive(Deserialize)]
struct SearchResponse {
    items: Vec<Repo>,
}

// Reusable client builder
fn build_client() -> Result<Client, reqwest::Error> {
    Client::builder().user_agent("gte/0.1").build()
}

// Get public repos for a user (with pagination support)
pub async fn fetch_repos(username: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = build_client()?;
    let mut all_repos = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page=100&page={}",
            username, page
        );

        let page_repos = client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Repo>>()
            .await?;

        if page_repos.is_empty() {
            break;
        }

        all_repos.extend(page_repos);
        page += 1;
    }

    Ok(all_repos)
}

// Get private repos for current user
pub async fn fetch_private_repos(token: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = build_client()?;
    let url = "https://api.github.com/user/repos?per_page=100&affiliation=owner,collaborator";

    client
        .get(url)
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Repo>>()
        .await
}

// Search repositories
pub async fn search_repos(query: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = build_client()?;
    let encoded_query = urlencoding::encode(query);
    let url = format!(
        "https://api.github.com/search/repositories?q={}&per_page=100",
        encoded_query
    );

    let search_response = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json::<SearchResponse>()
        .await?;

    Ok(search_response.items)
}

// Get repo issues
pub async fn get_repo_issues(repo: &str, token: &str) -> Result<Vec<Issue>, reqwest::Error> {
    let client = build_client()?;

    let url = format!("https://api.github.com/repos/{}/issues?state=all", repo);
    client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Issue>>()
        .await
}

// Get repo pull requests
pub async fn get_repo_prs(repo: &str, token: &str) -> Result<Vec<PR>, reqwest::Error> {
    let client = build_client()?;

    let url = format!("https://api.github.com/repos/{}/pulls?state=all", repo);
    client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<PR>>()
        .await
}
