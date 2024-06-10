use ratatui::widgets::ScrollbarState;
use serde::Deserialize;

use crate::config::ConfigToml;

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub info: Vec<String>,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Mode {
    #[serde(alias = "normal", alias = "NORMAL")]
    Normal,
    #[serde(alias = "info", alias = "INFO")]
    Info,
    #[serde(alias = "search", alias = "SEARCH")]
    Search,
    #[serde(alias = "command", alias = "COMMAND")]
    Command,
    Display,
}

pub enum Location {
    Search,
    Paclist,
    Pacinfo,
    Command,
}

pub struct App {
    pub mode: Mode,
    pub config: ConfigToml,
    pub packages: Vec<Package>,
    pub displayed_packages_indices: Vec<usize>,
    pub current_search: String,
    pub current_command: String,
    pub display_text: &'static str,
    pub list_scroll_state: ScrollbarState,
    pub info_scroll_state: ScrollbarState,
    pub search_cursor_index: usize,
    pub list_cursor_index: usize,
    pub info_cursor_index: usize,
    pub command_cursor_index: usize,
}
impl App {
    pub fn new(config: ConfigToml, packages: Vec<Package>) -> Self {
        Self {
            mode: config.operation.starting_mode,
            displayed_packages_indices: (0..packages.len()).collect(),
            packages,
            current_search: String::new(),
            current_command: String::new(),
            display_text: "",
            list_scroll_state: ScrollbarState::default(),
            info_scroll_state: ScrollbarState::default(),
            search_cursor_index: 0,
            list_cursor_index: 0,
            info_cursor_index: 0,
            command_cursor_index: 0,
            config,
        }
    }

    pub fn package_list_str(&self) -> String {
        // TODO inefficient
        // TODO replace with ui.rs display logic
        self.packages
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn print_package_list(&self) {
        println!("{}", self.package_list_str());
    }

    pub fn refresh_search(&mut self) {
        self.list_cursor_index = 0;
        self.info_cursor_index = 0;
        self.displayed_packages_indices = (0..self.packages.len())
            .filter(|index| self.packages[*index].name.contains(&self.current_search))
            .collect();
    }

    fn cursor_change(&mut self, location: &Location, change: i32) -> usize {
        let get_new_index =
            |index, length| (index as i32 + change).clamp(0, length as i32) as usize;
        let new_index: usize;
        match location {
            Location::Search => {
                new_index = get_new_index(self.search_cursor_index, self.current_search.len());
                self.search_cursor_index = new_index;
            }
            Location::Paclist => {
                let list_len = self.displayed_packages_indices.len();
                new_index = get_new_index(
                    self.list_cursor_index,
                    if list_len > 0 { list_len - 1 } else { 0 },
                );
                self.list_cursor_index = new_index;
            }
            Location::Pacinfo => {
                let info_len = self.packages[self.list_cursor_index].info.len();
                new_index = get_new_index(
                    self.info_cursor_index,
                    if info_len > 0 { info_len - 1 } else { 0 },
                );
                self.info_cursor_index = new_index;
            }
            Location::Command => {
                new_index = get_new_index(self.command_cursor_index, self.current_command.len());
                self.command_cursor_index = new_index;
            }
        }
        new_index
    }

    pub fn cursor_inc(&mut self, location: &Location) -> usize {
        self.cursor_change(location, 1)
    }

    pub fn cursor_dec(&mut self, location: &Location) -> usize {
        self.cursor_change(location, -1)
    }

    pub fn add_char(&mut self, c: char, location: &Location) -> Option<usize> {
        match location {
            Location::Search => {
                self.current_search.insert(self.search_cursor_index, c);
                Some(self.cursor_inc(&Location::Search))
            }
            Location::Command => {
                self.current_command.insert(self.command_cursor_index, c);
                Some(self.cursor_inc(&Location::Command))
            }
            _ => None,
        }
    }

    pub fn delete_char(&mut self, location: Location) -> Option<usize> {
        let new_str = move |mut str: String, cur_ind| {
            let before_char = str.chars().take(cur_ind - 1);
            let after_char = str.chars().skip(cur_ind);
            str = before_char.chain(after_char).collect();
            str
        };
        match location {
            Location::Search => {
                if self.search_cursor_index > 0 {
                    self.current_search =
                        new_str(self.current_search.clone(), self.search_cursor_index);
                    self.cursor_dec(&Location::Search);
                    Some(self.search_cursor_index)
                } else {
                    None
                }
            }
            Location::Command => {
                if self.command_cursor_index > 0 {
                    self.current_command =
                        new_str(self.current_command.clone(), self.command_cursor_index);
                    self.cursor_dec(&Location::Command);
                    Some(self.command_cursor_index)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn clear(&mut self, location: Location) {
        match location {
            Location::Search => {
                self.current_search = String::new();
                self.search_cursor_index = 0;
                self.refresh_search();
            }
            Location::Command => {
                self.current_command = String::new();
                self.command_cursor_index = 0;
            }
            _ => {}
        }
    }

    pub fn reset_info_scroll(&mut self) {
        self.info_cursor_index = 0;
        self.update_scroll_state(&Location::Pacinfo);
    }

    fn update_scroll_state(&mut self, location: &Location) {
        match location {
            Location::Paclist => {
                self.list_scroll_state = self.list_scroll_state.position(self.list_cursor_index);
            }
            Location::Pacinfo => {
                self.info_scroll_state = self.info_scroll_state.position(self.info_cursor_index);
            }
            _ => {}
        }
    }

    pub fn scroll_up(&mut self, location: &Location) {
        self.scroll(-1, location);
    }

    pub fn scroll_down(&mut self, location: &Location) {
        self.scroll(1, location);
    }

    fn scroll(&mut self, pos_change: i32, location: &Location) {
        self.cursor_change(location, pos_change);
        self.update_scroll_state(location);
    }

    pub fn goto_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.add_char(':', &Location::Command);
    }

    pub fn goto_display_mode(&mut self) {
        self.mode = Mode::Display;
        self.reset_info_scroll();
    }
}
