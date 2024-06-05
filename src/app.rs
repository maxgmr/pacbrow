#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub info: String,
}

pub enum Mode {
    Normal,
    Search,
    Command,
}

pub enum Location {
    Search,
    Paclist,
    Pacinfo,
    Command,
}

pub struct App {
    pub mode: Mode,
    pub packages: Vec<Package>,
    pub current_search: String,
    pub current_list: String,
    pub current_info: String,
    pub current_command: String,
    pub search_cursor_index: usize,
    pub list_cursor_index: usize,
    pub info_cursor_index: usize,
    pub command_cursor_index: usize,
}
impl App {
    pub fn new(packages: Vec<Package>) -> Self {
        Self {
            mode: Mode::Normal,
            packages,
            current_search: String::new(),
            current_list: String::new(),
            current_info: String::new(),
            current_command: String::new(),
            search_cursor_index: 0,
            list_cursor_index: 0,
            info_cursor_index: 0,
            command_cursor_index: 0,
        }
    }

    pub fn package_list_str(&self) -> String {
        // TODO inefficient
        self.packages
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn print_package_list(&self) {
        println!("{}", self.package_list_str());
    }

    fn cursor_change(&mut self, location: Location, change: i32) -> usize {
        let get_new_index =
            |index, length| (index as i32 + change).clamp(0, length as i32) as usize;
        let new_index: usize;
        match location {
            Location::Search => {
                new_index = get_new_index(self.search_cursor_index, self.current_search.len());
                self.search_cursor_index = new_index;
            }
            Location::Paclist => {
                new_index = get_new_index(self.list_cursor_index, self.current_list.len());
                self.list_cursor_index = new_index;
            }
            Location::Pacinfo => {
                new_index = get_new_index(self.info_cursor_index, self.current_info.len());
                self.info_cursor_index = new_index;
            }
            Location::Command => {
                new_index = get_new_index(self.command_cursor_index, self.current_command.len());
                self.command_cursor_index = new_index;
            }
        }
        new_index
    }

    pub fn cursor_inc(&mut self, location: Location) -> usize {
        self.cursor_change(location, 1)
    }

    pub fn cursor_dec(&mut self, location: Location) -> usize {
        self.cursor_change(location, -1)
    }

    pub fn add_char(&mut self, c: char, location: Location) -> Option<usize> {
        match location {
            Location::Search => {
                self.current_search.insert(self.search_cursor_index, c);
                Some(self.cursor_inc(Location::Search))
            }
            Location::Command => {
                self.current_command.insert(self.command_cursor_index, c);
                Some(self.cursor_inc(Location::Command))
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
                    self.cursor_dec(Location::Search);
                    Some(self.search_cursor_index)
                } else {
                    None
                }
            }
            Location::Command => {
                if self.command_cursor_index > 0 {
                    self.current_command =
                        new_str(self.current_command.clone(), self.command_cursor_index);
                    self.cursor_dec(Location::Command);
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
            }
            Location::Command => {
                self.current_command = String::new();
                self.command_cursor_index = 0;
            }
            _ => {}
        }
    }
}
