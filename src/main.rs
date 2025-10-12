mod github;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dotenvy::dotenv;
use github::fetch_repos;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::env;
use std::io;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // loads variables from .env

    let username = env::var("GITHUB_USERNAME").expect("missing GITHUB_USERNAME");
    let token = env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut interval = time::interval(Duration::from_secs(30));
    let mut repos = fetch_repos(username.as_str(), token.as_str()).await?;

    loop {
        ui::render_ui(&mut terminal, &repos)?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        tokio::select! {
            _ = interval.tick() => {
                if let Ok(new_repos) = fetch_repos(username.as_str(), token.as_str()).await {
                    repos = new_repos;
                }
            }
        }
    }

    // cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
