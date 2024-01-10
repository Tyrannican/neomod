mod state;
mod ui;

use std::path::PathBuf;

use anyhow::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Neomod {
    pub store: PathBuf,
    pub wow_loc: PathBuf,
    pub counter: i64,
    pub quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, neomod: &mut Neomod) -> Result<()> {
    // Messing around with some Ratatui
    loop {
        let _ = terminal.draw(|frame| ui::render_ui(frame, neomod));

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('j') => neomod.counter -= 1,
                KeyCode::Char('k') => neomod.counter += 1,
                KeyCode::Char('q') => neomod.quit = true,
                _ => {}
            }
        }

        if neomod.quit {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    startup()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut neomod = Neomod::default();
    run_app(&mut terminal, &mut neomod)?;
    shutdown()?;
    Ok(())
}
