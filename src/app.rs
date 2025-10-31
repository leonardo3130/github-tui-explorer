use crate::github::{fetch_repos, get_repo_issues, get_repo_prs, search_repos};
use ratatui::widgets::TableState;

use crate::models::{Issue, PR, Repo};
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    RepoList,
    RepoDetail,
    Search,
    IssuePopUp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Idle,
    Loading,
    Success,
    Error(String),
}

pub enum RepoDetailState {
    Details,
    Issues,
    PRs,
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
    pub prs: Vec<PR>,
    pub selected_pr: Option<PR>,
    pub detail_mode: RepoDetailState,

    // UI State
    pub table_state: TableState,
    pub loading_state: LoadingState,
    pub search_input: String,
    pub issue_table_state: TableState,
    pub pr_table_state: TableState,

    // scrolling
    pub scroll_offset: u16,
}

impl App {
    pub fn new(username: String, token: String) -> Result<Self, reqwest::Error> {
        let mut table_state = TableState::default();
        let mut issue_table_state = TableState::default();
        let mut pr_table_state = TableState::default();
        table_state.select(Some(0));
        issue_table_state.select(Some(0));
        pr_table_state.select(Some(0));

        Ok(Self {
            user: username,
            token: token,
            mode: AppMode::RepoList,
            should_quit: false,
            repos: Vec::new(),
            prs: Vec::new(),
            selected_repo: None,
            selected_issue: None,
            selected_pr: None,
            issues: Vec::new(),
            table_state,
            issue_table_state,
            pr_table_state,
            loading_state: LoadingState::Idle,
            search_input: String::new(),
            scroll_offset: 0,
            detail_mode: RepoDetailState::Details,
        })
    }

    // navigation
    fn select_next_in(state: &mut TableState, len: usize) {
        if len == 0 {
            return;
        }
        let i = match state.selected() {
            Some(i) if i + 1 < len => i + 1,
            _ => 0,
        };
        state.select(Some(i));
    }

    fn select_previous_in(state: &mut TableState, len: usize) {
        if len == 0 {
            return;
        }
        let i = match state.selected() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };
        state.select(Some(i));
    }

    pub fn next(&mut self) {
        match self.mode {
            AppMode::RepoList => Self::select_next_in(&mut self.table_state, self.repos.len()),
            AppMode::RepoDetail => match self.detail_mode {
                RepoDetailState::Details => {
                    self.scroll_offset = self.scroll_offset.saturating_add(1)
                }
                RepoDetailState::Issues => {
                    Self::select_next_in(&mut self.issue_table_state, self.issues.len())
                }
                RepoDetailState::PRs => {
                    Self::select_next_in(&mut self.pr_table_state, self.prs.len())
                }
            },
            AppMode::Search => {}
            AppMode::IssuePopUp => {}
        }
    }

    pub fn previous(&mut self) {
        match self.mode {
            AppMode::RepoList => Self::select_previous_in(&mut self.table_state, self.repos.len()),
            AppMode::RepoDetail => match self.detail_mode {
                RepoDetailState::Details => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1)
                }
                RepoDetailState::Issues => {
                    Self::select_previous_in(&mut self.issue_table_state, self.issues.len())
                }
                RepoDetailState::PRs => {
                    Self::select_previous_in(&mut self.pr_table_state, self.prs.len())
                }
            },
            AppMode::Search => {}
            AppMode::IssuePopUp => {}
        }
    }

    pub async fn select_current_repo(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.selected_repo = self.repos.get(i).cloned();
            self.load_selected_repo_issues().await;
            self.load_selected_repo_prs().await;
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

    pub async fn load_selected_repo_prs(&mut self) {
        if self.selected_repo.is_none() {
            return;
        }

        self.loading_state = LoadingState::Loading;

        match get_repo_prs(&self.selected_repo.clone().unwrap().full_name, &self.token).await {
            Ok(prs) => {
                self.prs = prs;
                self.loading_state = LoadingState::Success;
            }
            Err(e) => {
                self.loading_state = LoadingState::Error(e.to_string());
            }
        }
    }

    pub fn toggle_detail_mode(&mut self) {
        match self.detail_mode {
            RepoDetailState::Details => self.detail_mode = RepoDetailState::Issues,
            RepoDetailState::Issues => self.detail_mode = RepoDetailState::PRs,
            RepoDetailState::PRs => self.detail_mode = RepoDetailState::Details,
        }
    }
}
