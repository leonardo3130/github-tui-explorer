use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    // quit
    if key.code == KeyCode::Char('q') {
        return true;
    }

    match app.mode {
        AppMode::RepoList => handle_repo_list_keys(app, key),
        AppMode::RepoDetail => handle_repo_detail_keys(app, key),
        AppMode::Search => handle_search_keys(app, key),
    }

    false
}

pub fn handle_repo_list_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => app.next_repo(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_repo(),
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
            app.scroll_offset = app.scroll_offset.saturating_add(1)
        }
        KeyCode::Up | KeyCode::Char('k') => app.scroll_offset = app.scroll_offset.saturating_sub(1),
        KeyCode::Esc => app.back_to_list(),
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
        KeyCode::Enter => {
            // handled in the main loop with async
        }
        _ => {}
    }
}
