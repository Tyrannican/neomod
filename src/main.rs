mod state;
mod ui;

use state::Neomod;

use anyhow::Result;
use ratatui::prelude::*;

use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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
    loop {
        let _ = terminal.draw(|frame| ui::render_ui(frame, neomod));

        // Punt this to an update file
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
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
