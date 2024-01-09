use std::path::PathBuf;

use anyhow::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
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
        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            let counter_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let color = if neomod.counter >= 0 {
                Color::Green
            } else {
                Color::Red
            };
            let counter = Paragraph::new(Text::styled(
                format!("Counter: {}", neomod.counter),
                Style::default().fg(color),
            ))
            .block(counter_block)
            .alignment(Alignment::Center);

            frame.render_widget(counter, chunks[0]);
        });

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
