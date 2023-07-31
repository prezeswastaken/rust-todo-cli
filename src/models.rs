use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[derive(Insertable)]
#[diesel (table_name = crate::schema::tasks)] // Assuming "tasks" is the correct table name
pub struct Task {
    pub id: i32, // Change the type to i32 or i64 depending on your needs
    pub text: String, // Change the reference to owned String
    pub completed: bool,
}
