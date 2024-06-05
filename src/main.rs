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
mod ui;

use crate::{
    app::{App, Location, Mode},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app & run it
    // TODO get package list
    let mut app = App::new(Vec::new());
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
                    }
                    KeyCode::Char('s') | KeyCode::Char('i') => {
                        app.mode = Mode::Search;
                    }
                    KeyCode::Char('r') => {
                        app.clear(Location::Search);
                        app.mode = Mode::Search;
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
                        "q" => {
                            app.clear(Location::Command);
                            return Ok(false);
                        }
                        "qp" | "pq" => {
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
                        if app.current_command.is_empty() {
                            app.mode = Mode::Normal;
                        } else {
                            app.delete_char(Location::Command);
                        }
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
