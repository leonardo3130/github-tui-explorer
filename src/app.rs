use crate::{
    github::{fetch_repos, get_repo_issues, search_repos},
    models::Issue,
};
use ratatui::widgets::TableState;

use crate::models::Repo;
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    RepoList,
    RepoDetail,
    Search,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Idle,
    Loading,
    Success,
    Error(String),
}

pub struct App {
    // auth
    pub user: String,
    token: String,

    // navigation
    pub mode: AppMode,
    pub should_quit: bool,

    // data
    pub repos: Vec<Repo>,
    pub selected_repo: Option<Repo>,
    pub issues: Vec<Issue>,
    pub selected_issue: Option<Issue>,

    // UI State
    pub table_state: TableState,
    pub loading_state: LoadingState,
    pub search_input: String,
    pub issue_table_state: TableState,

    // scrolling
    pub scroll_offset: u16,
}

impl App {
    pub fn new(username: String, token: String) -> Result<Self, reqwest::Error> {
        let mut table_state = TableState::default();
        let mut issue_table_state = TableState::default();
        table_state.select(Some(0));
        issue_table_state.select(Some(0));

        Ok(Self {
            user: username,
            token: token,
            mode: AppMode::RepoList,
            should_quit: false,
            repos: Vec::new(),
            selected_repo: None,
            selected_issue: None,
            issues: Vec::new(),
            table_state,
            issue_table_state,
            loading_state: LoadingState::Idle,
            search_input: String::new(),
            scroll_offset: 0,
        })
    }

    // navigation
    pub fn next_repo(&mut self) {
        if self.repos.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.repos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_repo(&mut self) {
        if self.repos.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.repos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn next_issue(&mut self) {
        if self.issues.is_empty() {
            return;
        }
        let i = match self.issue_table_state.selected() {
            Some(i) => {
                if i >= self.issues.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.issue_table_state.select(Some(i));
    }

    pub fn previous_issue(&mut self) {
        if self.issues.is_empty() {
            return;
        }
        let i = match self.issue_table_state.selected() {
            Some(i) => {
                if i >= self.issues.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.issue_table_state.select(Some(i));
    }

    pub async fn select_current_repo(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.selected_repo = self.repos.get(i).cloned();
            self.load_selected_repo_issues().await;
            self.mode = AppMode::RepoDetail;

            self.scroll_offset = 0;
        }
    }

    pub fn select_current_issue(&mut self) {
        if let Some(i) = self.issue_table_state.selected() {
            self.selected_issue = self.issues.get(i).cloned();
        }
    }

    pub fn back_to_list(&mut self) {
        self.mode = AppMode::RepoList;
        self.selected_repo = None;
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = AppMode::Search;
        self.search_input.clear();
    }

    pub async fn load_user_repos(&mut self) {
        self.loading_state = LoadingState::Loading;

        match fetch_repos(&self.user).await {
            Ok(repos) => {
                self.repos = repos;
                self.loading_state = LoadingState::Success;
                if !self.repos.is_empty() {
                    self.table_state.select(Some(0));
                }
            }
            Err(e) => {
                self.loading_state = LoadingState::Error(e.to_string());
            }
        }
    }

    pub async fn search_repositories(&mut self) {
        if self.search_input.is_empty() {
            return;
        }

        self.loading_state = LoadingState::Loading;
        self.mode = AppMode::RepoList;

        match search_repos(&self.search_input).await {
            Ok(repos) => {
                self.repos = repos;
                self.loading_state = LoadingState::Success;
                if !self.repos.is_empty() {
                    self.table_state.select(Some(0));
                }
            }
            Err(e) => {
                self.loading_state = LoadingState::Error(e.to_string());
            }
        }
    }

    pub async fn load_selected_repo_issues(&mut self) {
        if self.selected_repo.is_none() {
            return;
        }

        self.loading_state = LoadingState::Loading;

        match get_repo_issues(&self.selected_repo.clone().unwrap().full_name, &self.token).await {
            Ok(issues) => {
                self.issues = issues;
                self.loading_state = LoadingState::Success;
            }
            Err(e) => {
                self.loading_state = LoadingState::Error(e.to_string());
            }
        }
    }
}
