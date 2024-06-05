use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn ui(f: &mut Frame, app: &App) {
    let search_info_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(search_info_layout[1]);

    // Temporary planning blocks
    let search = Paragraph::new("Search")
        .style(match app.mode {
            Mode::Search => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search, search_info_layout[0]);

    let pac_list = Paragraph::new("Package list")
        .style(match app.mode {
            Mode::Normal => Style::default().fg(Color::LightBlue),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Packages"));
    f.render_widget(pac_list, info_layout[0]);

    let info = Paragraph::new("Package info")
        .style(match app.mode {
            Mode::Normal => Style::default().fg(Color::LightGreen),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("TODO Selected Package Name"),
        );
    f.render_widget(info, info_layout[1]);

    let command_entry = Paragraph::new(app.current_command.to_string())
        .style(match app.mode {
            Mode::Command => Style::default().fg(Color::LightYellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Command <:>"));
    f.render_widget(command_entry, search_info_layout[2]);
}
