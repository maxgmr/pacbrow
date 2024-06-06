use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    symbols::scrollbar,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::{App, Mode, Package};

const NORMAL_COLOUR: Color = Color::Blue;
const INFO_COLOUR: Color = Color::Green;
const SEARCH_COLOUR: Color = Color::Cyan;
const COMMAND_COLOUR: Color = Color::Yellow;
const TEXT_COLOUR: Color = Color::White;

pub fn ui(f: &mut Frame, app: &mut App) {
    let selected_package: &Package = &app.packages[app.list_cursor_index];

    let mut i: usize = 0;
    let list_text = app
        .packages
        .iter()
        .map(|pkg| {
            i += 1;
            if (i - 1) == app.list_cursor_index {
                Line::from(Span::styled(
                    pkg.name.to_owned(),
                    Style::default()
                        .fg(NORMAL_COLOUR)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    pkg.name.to_owned(),
                    Style::default().fg(TEXT_COLOUR),
                ))
            }
        })
        .collect::<Vec<Line>>();

    let mut i: usize = 0;
    let info_text = selected_package
        .info
        .iter()
        .map(|line| {
            i += 1;
            if (i - 1) == app.info_cursor_index {
                Line::from(Span::styled(
                    line.to_owned(),
                    Style::default()
                        .fg(INFO_COLOUR)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    line.to_owned(),
                    Style::default().fg(TEXT_COLOUR),
                ))
            }
        })
        .collect::<Vec<Line>>();

    app.list_scroll_state = app.list_scroll_state.content_length(list_text.len());
    app.info_scroll_state = app.info_scroll_state.content_length(info_text.len());

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

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(search_info_layout[2]);

    let search = Paragraph::new(app.current_search.to_owned())
        .style(match app.mode {
            Mode::Search => Style::default().fg(SEARCH_COLOUR),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search, search_info_layout[0]);

    let pac_list = Paragraph::new(list_text.to_owned())
        .style(match app.mode {
            Mode::Normal => Style::default().fg(NORMAL_COLOUR),
            _ => Style::default(),
        })
        .wrap(Wrap { trim: false })
        .scroll((app.list_cursor_index as u16, 0))
        .block(Block::default().borders(Borders::ALL).title("Packages"));
    f.render_widget(pac_list, info_layout[0]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        info_layout[0].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.list_scroll_state,
    );

    let info = Paragraph::new(info_text.to_owned())
        .style(match app.mode {
            Mode::Info => Style::default().fg(INFO_COLOUR),
            _ => Style::default(),
        })
        .wrap(Wrap { trim: false })
        .scroll((app.info_cursor_index as u16, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(selected_package.name.to_owned()),
        );
    f.render_widget(info, info_layout[1]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        info_layout[1].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.info_scroll_state,
    );

    let command_entry = Paragraph::new(app.current_command.to_owned())
        .style(match app.mode {
            Mode::Command => Style::default().fg(COMMAND_COLOUR),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command (Type \":help\" for help)"),
        );
    f.render_widget(command_entry, bottom_layout[0]);

    let mode_info = (match app.mode {
        Mode::Normal => Paragraph::new("NORMAL").style(Style::default().fg(NORMAL_COLOUR)),
        Mode::Info => Paragraph::new("INFO").style(Style::default().fg(INFO_COLOUR)),
        Mode::Search => Paragraph::new("SEARCH").style(Style::default().fg(SEARCH_COLOUR)),
        Mode::Command => Paragraph::new("COMMAND").style(Style::default().fg(COMMAND_COLOUR)),
    })
    .block(Block::default().borders(Borders::ALL).title("Mode"));
    f.render_widget(mode_info, bottom_layout[1]);

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
