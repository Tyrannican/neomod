use crate::Neomod;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

fn render_header(frame: &mut Frame, section: Rect, neomod: &mut Neomod) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let header_text = Paragraph::new(Text::styled(
        "Neomod\nA DIY WoW Addon Manager",
        Style::default().fg(Color::LightBlue),
    ))
    .block(header_block)
    .alignment(Alignment::Center);

    frame.render_widget(header_text, section);
}

fn render_middle(frame: &mut Frame, section: Rect, neomod: &mut Neomod) {
    let middle_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    frame.render_widget(middle_block, section);
}

fn render_footer(frame: &mut Frame, section: Rect, neomod: &mut Neomod) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let keyhints = Span::styled("(q) to quit", Style::default().fg(Color::Red));
    let footer_text = Paragraph::new(Line::from(keyhints))
        .block(footer_block)
        .alignment(Alignment::Center);

    frame.render_widget(footer_text, section);
}

pub fn render_ui(frame: &mut Frame, neomod: &mut Neomod) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());
    let (header, middle, footer) = (layout[0], layout[1], layout[2]);

    render_header(frame, header, neomod);
    render_middle(frame, middle, neomod);
    render_footer(frame, footer, neomod);
}
