use crate::database_fetcher::establish_connection;
use crate::models::Task;
use crate::models::*;
use crate::schema;
use crate::schema::tasks;
use crate::schema::tasks::completed;
use diesel::connection::DefaultLoadingMode;
use diesel::prelude::*;

//I was here
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
//#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub current_position: i32,

    //pub connection: &'a SqliteConnection,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_position: 0,
        }
    }
}

impl App {
    pub fn fetch_data(&self) {
        let connection = &mut establish_connection();
        //let tasks = tasks::table.load(connection);
    }
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_up(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        } else {
            self.current_position = 7;
        }
    }

    pub fn move_down(&mut self) {
        if self.current_position < 7 {
            self.current_position += 1;
        } else {
            self.current_position = 0;
        }
    }
}
