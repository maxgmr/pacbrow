use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode, Package};

pub fn ui(f: &mut Frame, app: &App) {
    let selected_package: &Package = &app.packages[app.list_cursor_index];

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
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(search_info_layout[1]);

    let search = Paragraph::new(app.current_search.to_owned())
        .style(match app.mode {
            Mode::Search => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search, search_info_layout[0]);

    let pac_list = Paragraph::new(app.package_list_str())
        .style(match app.mode {
            Mode::Normal => Style::default().fg(Color::LightBlue),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Packages"));
    f.render_widget(pac_list, info_layout[0]);

    let info = Paragraph::new(selected_package.info.to_owned())
        .style(match app.mode {
            Mode::Info => Style::default().fg(Color::LightGreen),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(selected_package.name.to_owned()),
        );
    f.render_widget(info, info_layout[1]);

    let command_entry = Paragraph::new(app.current_command.to_owned())
        .style(match app.mode {
            Mode::Command => Style::default().fg(Color::LightYellow),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command (Type \":help\" for help)"),
        );
    f.render_widget(command_entry, search_info_layout[2]);

    // Render the cursor
    match app.mode {
        Mode::Command => f.set_cursor(
            search_info_layout[2].x + app.command_cursor_index as u16 + 1,
            search_info_layout[2].y + 1,
        ),
        Mode::Search => f.set_cursor(
            search_info_layout[0].x + app.search_cursor_index as u16 + 1,
            search_info_layout[0].y + 1,
        ),
        _ => {}
    }
}
