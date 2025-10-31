use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    if key.code == KeyCode::Char('q') {
        return true;
    }

    match app.mode {
        AppMode::RepoList => {
            if key.code == KeyCode::Enter {
                app.select_current_repo().await;
            } else {
                handle_repo_list_keys(app, key);
            }
        }
        AppMode::RepoDetail => handle_repo_detail_keys(app, key),
        AppMode::Search => {
            if key.code == KeyCode::Enter {
                app.search_repositories().await;
            } else {
                handle_search_keys(app, key);
            }
        }
        AppMode::IssuePopUp => handle_issue_popup_keys(app, key),
    }

    false
}

pub fn handle_issue_popup_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.back_to_list(),
        _ => {}
    }
}

pub fn handle_repo_list_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.previous(),
        KeyCode::Char('/') => app.enter_search_mode(),
        _ => {}
    }
}

fn handle_issue_list_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('/') => app.enter_search_mode(),
        _ => {}
    }
}

pub fn handle_repo_detail_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Esc => app.back_to_list(),
        KeyCode::Tab => app.toggle_detail_mode(),
        _ => handle_issue_list_keys(app, key),
    }
}

pub fn handle_search_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => app.search_input.push(c),
        KeyCode::Backspace => {
            app.search_input.pop();
        }
        KeyCode::Esc => app.back_to_list(),
        _ => {}
    }
}
