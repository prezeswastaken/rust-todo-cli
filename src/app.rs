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
pub struct App {
    pub running: bool,

    pub current_position: i32,
    pub tasks: Vec<Task>,

    pub connection: Option<SqliteConnection>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_position: 0,
            tasks: Vec::new(),
            connection: None,
        }
    }
}

impl App {
    pub fn establish_connection_app(&mut self) -> i32 {
        let connection = establish_connection();

        match &mut self.connection {
            Some(_) => return 0,
            None => self.connection = Some(connection),
        }

        return 0;
    }
    pub fn fetch_data(&mut self) -> i32 {
        if self.connection.is_none() {
            return 1;
        }
        let connection = self.connection.as_mut().unwrap();
        let tasks: Vec<Task> = tasks::table
            .load(connection)
            .expect("coulnd load data from database");

        self.tasks = tasks;
        return 0;
    }

    pub fn create_task(&mut self, text: String) -> i32 {
        const MAX_TASKS_COUNT: usize = 8;
        self.fetch_data();
        if self.tasks.len() >= MAX_TASKS_COUNT {
            return 0;
        }

        let connection = self.connection.as_mut().unwrap();

        let new_task = NewTask {
            text: &format!("{}", text),
        };
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(connection)
            .expect("Error saving new post");
        self.fetch_data();
        1
    }

    pub fn get_current_task_id(&mut self) -> i32 {
        let current_task: Option<&Task> = self.tasks.get(self.current_position as usize);
        if current_task.is_none() {
            return 0;
        }

        let id = current_task.unwrap().id;

        return id;
    }

    pub fn mark_current_task_as_completed(&mut self) -> i32 {
        let id = self.get_current_task_id();

        let current_task: Option<&Task> = self.tasks.get(self.current_position as usize);

        if current_task.is_none() {
            return 1;
        }

        if current_task.unwrap().completed == false {
            let updated_row = diesel::update(tasks::table.filter(tasks::id.eq(id)))
                .set(completed.eq(true))
                .execute(self.connection.as_mut().unwrap());
        } else {
            let updated_row = diesel::update(tasks::table.filter(tasks::id.eq(id)))
                .set(completed.eq(false))
                .execute(self.connection.as_mut().unwrap());
        }

        self.fetch_data();
        return 0;
    }

    pub fn delete_current_task(&mut self) -> i32 {
        if self.tasks.len() == 1 {
            return 0;
        }
        let id_to_delete = self.get_current_task_id();
        let connection = self.connection.as_mut().unwrap();
        diesel::delete(tasks::table.filter(tasks::id.eq(id_to_delete)))
            .execute(connection)
            .expect("Error deleting task");
        self.fetch_data();

        if self.current_position > (self.tasks.len() - 1) as i32 {
            if self.tasks.len() == 0 {
                self.current_position = 0;
                return 0;
            }

            self.current_position = (self.tasks.len() - 1) as i32;
        }
        return 0;
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
        if (self.current_position - 1) < 0 {
            self.current_position = (self.tasks.len() - 1) as i32;
        } else {
            self.current_position = self.current_position - 1;
        }
    }
    pub fn move_down(&mut self) {
        if (self.current_position + 1) as usize >= self.tasks.len() {
            self.current_position = 0;
        } else {
            self.current_position += 1;
        }
    }
}
