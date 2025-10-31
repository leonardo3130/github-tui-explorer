use std::str::FromStr;

use crate::app::App;
use crate::app::AppMode;
use crate::app::LoadingState;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, Wrap},
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn render_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10), // Header
            Constraint::Min(0),         // Main content
            Constraint::Length(3),      // Footer
        ])
        .split(f.area());

    render_header(f, chunks[0], app);

    match app.mode {
        AppMode::RepoList => render_repo_list(f, chunks[1], app),
        AppMode::RepoDetail => render_repo_detail(f, chunks[1], app),
        AppMode::Search => render_search_input(f, chunks[1], app),
        AppMode::IssuePopUp => render_issue_popup(f, chunks[1], app),
    }

    render_footer(f, chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    if app.mode == AppMode::IssuePopUp {
        return;
    }

    let title = match app.mode {
        AppMode::RepoList => {
            Line::from(format!("GitHub Repos - {}", app.user)).alignment(Alignment::Center)
        }
        AppMode::RepoDetail => Line::from("Repository Details").alignment(Alignment::Center),
        AppMode::Search => Line::from("Search Repositories").alignment(Alignment::Center),
        AppMode::IssuePopUp => Line::from("_").alignment(Alignment::Center),
    };

    let header = Paragraph::new(title)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::new()
                .padding(Padding::new(
                    0,               // left
                    0,               // right
                    area.height / 4, // top
                    0,               // bottom
                ))
                .borders(Borders::ALL),
        );

    f.render_widget(header, area);
}

fn render_repo_list(f: &mut Frame, area: Rect, app: &mut App) {
    match &app.loading_state {
        LoadingState::Loading => {
            let loading = Paragraph::new("Loading repositories...")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Repositories"));
            f.render_widget(loading, area);
        }
        LoadingState::Error(err) => {
            let error = Paragraph::new(format!("Error: {}", err))
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, area);
        }
        _ => {
            if app.repos.is_empty() {
                let empty = Paragraph::new("No repositories found")
                    .style(Style::default().fg(Color::Gray))
                    .block(Block::default().borders(Borders::ALL).title("Repositories"));
                f.render_widget(empty, area);
                return;
            }

            let header = Row::new(vec!["Name", "Stars", "Language", "Description"])
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .bottom_margin(1);

            let rows = app.repos.iter().map(|repo| {
                Row::new(vec![
                    repo.full_name.clone(),
                    repo.stargazers_count.to_string(),
                    repo.language.as_deref().unwrap_or("N/A").to_string(),
                    repo.description.as_deref().unwrap_or("N/A").to_string(),
                ])
            });

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(20),
                    Constraint::Length(8),
                    Constraint::Length(15),
                    Constraint::Percentage(57),
                ],
            )
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Repositories"))
            .row_highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

            f.render_stateful_widget(table, area, &mut app.table_state);
        }
    }
}

fn render_repo_detail(f: &mut Frame, area: Rect, app: &mut App) {
    match &app.loading_state {
        LoadingState::Loading => {
            let loading = Paragraph::new("Loading repositories...")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Repositories"));
            f.render_widget(loading, area);
        }
        LoadingState::Error(err) => {
            let error = Paragraph::new(format!("Error: {}", err))
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, area);
        }
        _ => {
            if let Some(repo) = &app.selected_repo {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(20), // repo info
                        Constraint::Percentage(40), // issues
                        Constraint::Percentage(40), // PRs
                    ])
                    .split(area);

                let details = format!(
                    "Name: {}\n\
                        Stars: ⭐ {}\n\
                        Forks: 🍴 {}\n\
                        Language: {}\n\
                        Created: {}\n\
                        Updated: {}\n\n\
                        Description:\n{}\n\n\
                        URL: {}",
                    repo.full_name,
                    repo.stargazers_count,
                    repo.forks_count,
                    repo.language.as_deref().unwrap_or("N/A"),
                    repo.created_at,
                    repo.updated_at,
                    repo.description.as_deref().unwrap_or("No description"),
                    repo.html_url
                );

                let paragraph = Paragraph::new(details)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Repository Details"),
                    )
                    .wrap(Wrap { trim: true })
                    .scroll((app.scroll_offset, 0));

                let issue_header = Row::new(vec!["Title", "Body", "State", "URL", "Labels"])
                    .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .bottom_margin(1)
                    .top_margin(2);

                let issues = app.issues.clone();

                let issue_rows = issues.iter().map(|issue| {
                    Row::new(vec![
                        issue.title.clone(),
                        issue.body.clone().unwrap_or(String::from("N/A")),
                        issue.state.clone(),
                        issue.html_url.clone(),
                        issue
                            .labels
                            .iter()
                            .map(|label| label.name.clone())
                            .collect::<Vec<String>>()
                            .join(", "),
                    ])
                });

                let issue_table = Table::new(
                    issue_rows,
                    [
                        Constraint::Percentage(10),
                        Constraint::Length(40),
                        Constraint::Length(6),
                        Constraint::Percentage(32),
                        Constraint::Percentage(12),
                    ],
                )
                .header(issue_header)
                .block(Block::default().borders(Borders::ALL).title("Issues"))
                .row_highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">! ");

                let pr_header = Row::new(vec!["Title", "Body", "State", "URL"])
                    .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .bottom_margin(1)
                    .top_margin(2);

                let prs = app.prs.clone();

                let pr_rows = prs.iter().map(|pr| {
                    Row::new(vec![
                        pr.title.clone(),
                        pr.body.clone().unwrap_or(String::from("N/A")),
                        pr.state.clone(),
                        pr.html_url.clone(),
                    ])
                });

                let pr_table = Table::new(
                    pr_rows,
                    [
                        Constraint::Percentage(10),
                        Constraint::Length(50),
                        Constraint::Length(8),
                        Constraint::Percentage(32),
                    ],
                )
                .header(pr_header)
                .block(Block::default().borders(Borders::ALL).title("PRs"))
                .row_highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">! ");

                f.render_widget(paragraph, chunks[0]);
                f.render_stateful_widget(issue_table, chunks[1], &mut app.issue_table_state);
                f.render_stateful_widget(pr_table, chunks[2], &mut app.pr_table_state);
            }
        }
    }
}

fn render_search_input(f: &mut Frame, area: Rect, app: &App) {
    let input = Paragraph::new(app.search_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search Query"));

    f.render_widget(input, area);
}

fn render_issue_popup(f: &mut Frame, area: Rect, app: &App) {
    let area = popup_area(area, 80, 60);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    let issue = app.selected_issue.clone().unwrap();
    let details = format!(
        "Title: {}\n\
        State: {}\n\
        Body: {}\n\
        URL: {}",
        issue.title,
        issue.state,
        issue.body.unwrap_or(String::from("N/A")),
        issue.html_url,
    );

    let paragraph = Paragraph::new(details)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Issue #{} details", issue.number)),
        )
        .wrap(Wrap { trim: true });

    let mut spans: Vec<Span> = Vec::new();

    for (i, label) in issue.labels.iter().enumerate() {
        if i > 0 {
            spans.push(Span::from(","))
        }

        let ratatui_color = Color::from_str(
            label
                .color
                .clone()
                .unwrap_or(String::from("white"))
                .as_str(),
        );

        let mut final_color = Color::White;

        match ratatui_color {
            Ok(color) => final_color = color,
            Err(_) => {}
        }

        spans.push(Span::from(label.name.clone()).style(Style::default().fg(final_color)))
    }

    let labels_line = Line::from(spans);
    let labels_text = Text::from(labels_line);
    let labels_paragraph = Paragraph::new(labels_text)
        .block(Block::default().borders(Borders::ALL).title("Issue Labels"));

    f.render_widget(paragraph, chunks[0]);
    f.render_widget(labels_paragraph, chunks[1]);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.mode {
        AppMode::RepoList => "↑/↓: Navigate | Enter: View Details | /: Search | q: Quit",
        AppMode::RepoDetail => {
            "↑/↓: Scroll | Esc: Back | q: Quit | Tab: toggle between repo issues, PRs and details"
        }
        AppMode::Search => "Type to search | Enter: Execute | Esc: Cancel",
        AppMode::IssuePopUp => "↑/↓: Scroll | Esc: Back | q: Quit",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
