use diesel::prelude::*;
use crate::schema::tasks;


#[derive(Queryable, Selectable)]
#[diesel (table_name = tasks)] // Assuming "tasks" is the correct table name
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]                                          // 
pub struct Task {
    pub id: i32, 
    pub text: String, 
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub text: &'a str,
}
