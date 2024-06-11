use cli_clipboard::set_contents;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

mod app;
mod config;
mod display_texts;
mod paclist;
mod ui;

use crate::{
    app::{App, Location, Mode},
    config::read_config,
    paclist::get_package_list,
    ui::ui,
};

const TICK_RATE_MS: u64 = 250;

// Planned features
// TODO filters; AUR only, orphans only, explicitly installed only, etc.
// TODO non-latin characters
// TODO list number of results and index of current result
// TODO sort by size, date installed, etc.
// TODO search by fields
// TODO allow copying of line in info mode to clipboard
// TODO FIX: Display mode can only scroll as far as the current package
fn main() -> Result<(), Box<dyn Error>> {
    // Load config
    let config_toml = read_config()?;

    // Get list of packages
    let package_list = get_package_list()?;

    if package_list.is_empty() {
        eprintln!("Unable to get package list. Please ensure you are using pacman as your package manager and that it is working properly.");
    }

    // Terminal setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app & run it
    let mut app = App::new(config_toml, package_list);
    let res = run_app(&mut terminal, &mut app, Duration::from_millis(TICK_RATE_MS));

    // Restore terminal after app execution complete
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    match res {
        Ok(do_print) => {
            if do_print {
                app.print_package_list();
            }
        }
        Err(err) => {
            eprintln!("{err:?}")
        }
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<bool> {
    let mut last_tick = Instant::now();
    // Main loop
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)? {
            // Handle keypresses
            // TODO make this cleaner
            // TODO add specific methods for switching modes in App
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char(':') => {
                            app.goto_command_mode();
                        }
                        KeyCode::Char('s') => {
                            app.mode = Mode::Search;
                        }
                        KeyCode::Char('r') => {
                            app.clear(Location::Search);
                            app.mode = Mode::Search;
                        }
                        // Yank selected package name
                        KeyCode::Char('y') => {
                            set_contents(app.current_paclist[app.list_cursor_index].to_owned())
                                .unwrap();
                        }
                        // Yank current package list
                        KeyCode::Char('Y') => {
                            set_contents(app.current_paclist.join("\n").to_owned()).unwrap();
                        }
                        // Scroll up package list
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_up(&Location::Paclist);
                            app.reset_info_scroll();
                        }
                        // Scroll down package list
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_down(&Location::Paclist);
                            app.reset_info_scroll();
                        }
                        KeyCode::Char('u') => {
                            app.scroll_up_fast(&Location::Paclist);
                            app.reset_info_scroll();
                        }
                        KeyCode::Char('d') => {
                            app.scroll_down_fast(&Location::Paclist);
                            app.reset_info_scroll();
                        }
                        KeyCode::Char('g') => {
                            app.cursor_jump(&Location::Paclist, 0);
                        }
                        KeyCode::Char('G') => {
                            app.cursor_jump(&Location::Paclist, app.current_paclist.len() - 1);
                        }
                        // Enter info mode for the currently selected package
                        KeyCode::Char('l')
                        | KeyCode::Char('i')
                        | KeyCode::Right
                        | KeyCode::Enter => {
                            app.mode = Mode::Info;
                        }
                        _ => {}
                    },
                    Mode::Info => match key.code {
                        KeyCode::Char(':') => {
                            app.goto_command_mode();
                        }
                        KeyCode::Char('s') => {
                            app.mode = Mode::Search;
                        }
                        KeyCode::Char('r') => {
                            app.clear(Location::Search);
                            app.mode = Mode::Search;
                        }
                        // Yank current line
                        KeyCode::Char('y') => {
                            set_contents(app.current_pacinfo[app.info_cursor_index].to_owned())
                                .unwrap();
                        }
                        // Yank package info
                        KeyCode::Char('Y') => {
                            set_contents(app.current_pacinfo.join("\n").to_owned()).unwrap();
                        }
                        KeyCode::Char('u') => {
                            app.scroll_up_fast(&Location::Pacinfo);
                        }
                        KeyCode::Char('d') => {
                            app.scroll_down_fast(&Location::Pacinfo);
                        }
                        // Exit info mode
                        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('n') | KeyCode::Left => {
                            app.mode = Mode::Normal;
                        }
                        // Scroll up package info
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_up(&Location::Pacinfo);
                        }
                        // Scroll down package info
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_down(&Location::Pacinfo);
                        }
                        KeyCode::Char('g') => {
                            app.cursor_jump(&Location::Pacinfo, 0);
                        }
                        KeyCode::Char('G') => {
                            app.cursor_jump(&Location::Pacinfo, app.current_pacinfo.len() - 1);
                        }
                        _ => {}
                    },
                    Mode::Command => match key.code {
                        // Exit command entry mode
                        KeyCode::Esc => {
                            app.clear(Location::Command);
                            app.mode = Mode::Normal;
                        }
                        // User submits typed command
                        KeyCode::Enter => match app.current_command.as_str() {
                            ":help" | ":h" => {
                                app.clear(Location::Command);
                                app.goto_display_mode(display_texts::HELP_TEXT);
                            }
                            ":quit" | ":q" => {
                                app.clear(Location::Command);
                                return Ok(false);
                            }
                            ":p" | ":print" => {
                                app.clear(Location::Command);
                                app.print_package_list();
                                app.mode = Mode::Normal;
                            }
                            ":qp" | ":pq" => {
                                app.clear(Location::Command);
                                return Ok(true);
                            }
                            ":c" | "commands" => {
                                app.clear(Location::Command);
                                app.goto_display_mode(display_texts::COMMAND_LIST);
                            }
                            ":dedication" | ":love" => {
                                app.clear(Location::Command);
                                app.goto_display_mode(
                                    "Made by Max Gilmour\n\nTo Michayla, who always listens",
                                );
                            }
                            // TODO user feedback on unknown command
                            _ => {
                                app.clear(Location::Command);
                                app.mode = Mode::Normal;
                            }
                        },
                        // User is deleting something; if already empty exit command mode
                        KeyCode::Backspace => {
                            if app.current_command.len() == 1 {
                                app.clear(Location::Command);
                                app.mode = Mode::Normal;
                            } else {
                                app.delete_char(Location::Command);
                            }
                        }
                        // Move cursor to the left
                        KeyCode::Left => {
                            app.cursor_dec(&Location::Command);
                        }
                        // Move cursor to the right
                        KeyCode::Right => {
                            app.cursor_inc(&Location::Command);
                        }
                        // User is typing something
                        KeyCode::Char(new_char) => {
                            app.add_char(new_char, &Location::Command);
                        }
                        _ => {}
                    },
                    Mode::Search => match key.code {
                        KeyCode::Down | KeyCode::Esc | KeyCode::Enter => {
                            app.mode = Mode::Normal;
                        }
                        KeyCode::Char(':') => {
                            // Only enter the colon if config allows; otherwise, switch to command
                            // mode.
                            if app.config.operation.allow_colon_in_search {
                                app.add_char(':', &Location::Search);
                                app.refresh_search();
                            } else {
                                app.mode = Mode::Command;
                                app.add_char(':', &Location::Command);
                            }
                        }
                        // User is deleting something; if already empty exit search mode
                        KeyCode::Backspace => {
                            if app.current_search.is_empty() {
                                app.mode = Mode::Normal;
                            } else {
                                app.delete_char(Location::Search);
                                app.refresh_search();
                            }
                        }
                        // Move cursor to the left
                        KeyCode::Left => {
                            app.cursor_dec(&Location::Search);
                        }
                        // Move cursor to the right
                        KeyCode::Right => {
                            app.cursor_inc(&Location::Search);
                        }
                        // User is typing something
                        KeyCode::Char(new_char) => {
                            app.add_char(new_char, &Location::Search);
                            app.refresh_search();
                        }
                        _ => {}
                    },
                    Mode::Display => match key.code {
                        KeyCode::Esc => {
                            app.leave_display_mode(Mode::Normal);
                        }
                        KeyCode::Char(':') => {
                            app.leave_display_mode(Mode::Command);
                            app.goto_command_mode();
                        }
                        // Scroll up
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.scroll_up(&Location::Pacinfo);
                        }
                        // Scroll down
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.scroll_down(&Location::Pacinfo);
                        }
                        KeyCode::Char('u') => {
                            app.scroll_up_fast(&Location::Pacinfo);
                        }
                        KeyCode::Char('d') => {
                            app.scroll_down_fast(&Location::Pacinfo);
                        }
                        KeyCode::Char('g') => {
                            app.cursor_jump(&Location::Pacinfo, 0);
                        }
                        KeyCode::Char('G') => {
                            app.cursor_jump(&Location::Pacinfo, app.current_pacinfo.len() - 1);
                        }
                        _ => {}
                    },
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
