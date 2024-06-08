use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::{Modifier, Style},
    symbols::scrollbar,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::{App, Mode, Package};

pub fn ui(f: &mut Frame, app: &mut App) {
    let selected_package: Option<&Package> = if !app.displayed_packages_indices.is_empty() {
        Some(&app.packages[app.displayed_packages_indices[app.list_cursor_index]])
    } else {
        None
    };

    let mut i: usize = 0;
    let list_text = match selected_package {
        Some(_) => app
            .displayed_packages_indices
            .iter()
            .map(|index| {
                i += 1;
                if (i - 1) == app.list_cursor_index {
                    Line::from(Span::styled(
                        app.packages[*index].name.to_owned(),
                        Style::default()
                            .fg(app.config.colours.normal)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(Span::styled(
                        app.packages[*index].name.to_owned(),
                        Style::default().fg(app.config.colours.text),
                    ))
                }
            })
            .collect::<Vec<Line>>(),
        None => vec![Line::from("")],
    };

    let mut i: usize = 0;
    let info_text = match selected_package {
        Some(pkg) => pkg
            .info
            .iter()
            .map(|line| {
                i += 1;
                if (i - 1) == app.info_cursor_index {
                    Line::from(Span::styled(
                        line.to_owned(),
                        Style::default()
                            .fg(app.config.colours.info)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(Span::styled(
                        line.to_owned(),
                        Style::default().fg(app.config.colours.text),
                    ))
                }
            })
            .collect::<Vec<Line>>(),
        None => vec![Line::from("")],
    };

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
            Mode::Search => Style::default().fg(app.config.colours.search),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(search, search_info_layout[0]);

    let pac_list = Paragraph::new(list_text.to_owned())
        .style(match app.mode {
            Mode::Normal => Style::default().fg(app.config.colours.normal),
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
            Mode::Info => Style::default().fg(app.config.colours.info),
            _ => Style::default(),
        })
        .wrap(Wrap { trim: false })
        .scroll((app.info_cursor_index as u16, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(match selected_package {
                    Some(pkg) => pkg.name.to_owned(),
                    None => String::from(""),
                }),
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
            Mode::Command => Style::default().fg(app.config.colours.command),
            _ => Style::default(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command (Type \":help\" for help)"),
        );
    f.render_widget(command_entry, bottom_layout[0]);

    let mode_info = (match app.mode {
        Mode::Normal => {
            Paragraph::new("NORMAL").style(Style::default().fg(app.config.colours.normal))
        }
        Mode::Info => Paragraph::new("INFO").style(Style::default().fg(app.config.colours.info)),
        Mode::Search => {
            Paragraph::new("SEARCH").style(Style::default().fg(app.config.colours.search))
        }
        Mode::Command => {
            Paragraph::new("COMMAND").style(Style::default().fg(app.config.colours.command))
        }
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
