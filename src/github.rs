use reqwest::Client;
use serde::Deserialize; // adjust path as needed

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
}

pub async fn fetch_repos(username: &str, token: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let client = Client::builder()
        .user_agent("gte/0.1") // important for GitHub API
        .build()?;

    let mut repos: Vec<Repo> = Vec::new();
    let mut page: u32 = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page=100&page={}",
            username, page
        );

        let resp = client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?
            .error_for_status()?; // convert HTTP error codes into Err

        // Parse JSON for this page
        let mut page_repos = resp.json::<Vec<Repo>>().await?;

        if page_repos.is_empty() {
            break;
        }

        repos.append(&mut page_repos);
        page += 1;
    }

    Ok(repos)
}
