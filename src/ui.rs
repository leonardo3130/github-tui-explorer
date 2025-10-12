use crate::github::Repo;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};

pub fn render_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    repos: &[Repo],
) -> anyhow::Result<()> {
    terminal.draw(|f| {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)].as_ref())
            .split(size);

        let header = Row::new(vec!["Name", "⭐ Stars", "🍴 Forks", "🐛 Issues"])
            .style(Style::default().fg(Color::Yellow));

        let rows = repos.iter().map(|r| {
            Row::new(vec![
                r.name.clone(),
                r.stargazers_count.to_string(),
                r.forks_count.to_string(),
                r.open_issues_count.to_string(),
            ])
        });

        let widths = [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths).header(header).block(
            Block::default()
                .borders(Borders::ALL)
                .title("GitHub Repositories"),
        );

        f.render_widget(table, chunks[0]);
    })?;
    Ok(())
}
