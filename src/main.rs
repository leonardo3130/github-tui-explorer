use crossterm::{
    event::KeyCode,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crossterm::event;
use crossterm::event::Event;

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;
use tokio;

pub mod app;
pub mod events;
pub mod github;
pub mod models;
pub mod ui;

use app::App;
use app::AppMode;

use dotenvy::dotenv;
use std::env;

async fn run_app(username: &str, token: &str) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(username.to_string(), token.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Load initial data
    app.load_user_repos().await;

    // Main loop
    loop {
        terminal.draw(|f| ui::render_ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }

                match app.mode {
                    AppMode::RepoList => events::handle_repo_list_keys(&mut app, key),
                    AppMode::RepoDetail => events::handle_repo_detail_keys(&mut app, key),
                    AppMode::Search => {
                        if key.code == KeyCode::Enter {
                            app.search_repositories().await;
                        } else {
                            events::handle_search_keys(&mut app, key);
                        }
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // loads variables from .env

    let username = env::var("GITHUB_USERNAME").expect("missing GITHUB_USERNAME");
    let token = env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN");

    run_app(username.as_str(), token.as_str()).await?;
    Ok(())
}
