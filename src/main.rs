use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use std::{error::Error, io};

mod app;
mod paclist;
mod ui;

use crate::{
    app::{App, Location, Mode},
    paclist::get_package_list,
    ui::ui,
};

// Planned features
// TODO filters; AUR only, orphans only, explicitly installed only, etc.
// TODO non-latin characters
fn main() -> Result<(), Box<dyn Error>> {
    // Get list of packages
    let package_list = get_package_list()?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app & run it
    let mut app = App::new(package_list);
    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // Main loop
    loop {
        terminal.draw(|f| ui(f, app))?;

        // Handle keypresses
        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char(':') => {
                        app.mode = Mode::Command;
                        app.add_char(':', Location::Command);
                    }
                    KeyCode::Char('s') | KeyCode::Char('i') => {
                        app.mode = Mode::Search;
                    }
                    KeyCode::Char('r') => {
                        app.clear(Location::Search);
                        app.mode = Mode::Search;
                    }
                    // Scroll up package list
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.info_cursor_index = 0;
                        app.cursor_dec(Location::Paclist);
                        app.list_scroll_state =
                            app.list_scroll_state.position(app.list_cursor_index);
                    }
                    // Scroll down package list
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.info_cursor_index = 0;
                        app.cursor_inc(Location::Paclist);
                        app.list_scroll_state =
                            app.list_scroll_state.position(app.list_cursor_index);
                    }
                    // Enter info mode for the currently selected package
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                        app.mode = Mode::Info;
                    }
                    _ => {}
                },
                Mode::Info => match key.code {
                    // Exit info mode
                    KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => {
                        app.mode = Mode::Normal;
                    }
                    // Scroll up package info
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.cursor_dec(Location::Pacinfo);
                        app.info_scroll_state =
                            app.info_scroll_state.position(app.info_cursor_index);
                    }
                    // Scroll down package info
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.cursor_inc(Location::Pacinfo);
                        app.info_scroll_state =
                            app.info_scroll_state.position(app.info_cursor_index);
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
                            // TODO show help page
                            app.clear(Location::Command);
                            app.mode = Mode::Normal;
                        }
                        ":q" => {
                            app.clear(Location::Command);
                            return Ok(false);
                        }
                        ":qp" | ":pq" => {
                            app.clear(Location::Command);
                            return Ok(true);
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
                        app.cursor_dec(Location::Command);
                    }
                    // Move cursor to the right
                    KeyCode::Right => {
                        app.cursor_inc(Location::Command);
                    }
                    // User is typing something
                    KeyCode::Char(new_char) => {
                        app.add_char(new_char, Location::Command);
                    }
                    _ => {}
                },
                Mode::Search => match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.mode = Mode::Normal;
                    }
                    // User is deleting something; if already empty exit search mode
                    KeyCode::Backspace => {
                        if app.current_search.is_empty() {
                            app.mode = Mode::Normal;
                        } else {
                            app.delete_char(Location::Search);
                        }
                    }
                    // Move cursor to the left
                    KeyCode::Left => {
                        app.cursor_dec(Location::Search);
                    }
                    // Move cursor to the right
                    KeyCode::Right => {
                        app.cursor_inc(Location::Search);
                    }
                    // User is typing something
                    KeyCode::Char(new_char) => {
                        app.add_char(new_char, Location::Search);
                    }
                    _ => {}
                },
            }
        }
    }
}
