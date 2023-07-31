use crate::database_fetcher::establish_connection;
use crate::models::NewTask;
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
    pub running: bool,

    pub current_position: i32,
    pub tasks: Vec<Task>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_position: 0,
            tasks: Vec::new(),
        }
    }
}

impl App {
    pub fn fetch_data(&mut self) {
        let connection = &mut establish_connection();
        let tasks: Vec<Task> = tasks::table
            .load(connection)
            .expect("coulnd load data from database");

        self.tasks = tasks;
    }

    pub fn create_fake_tasks(&mut self, text: String) {
        let connection = &mut establish_connection();

        let new_task = NewTask { text: &format!("{}", text)};
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(connection)
            .expect("Error saving new post");
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
