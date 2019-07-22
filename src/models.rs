use super::schema::tasks;
use chrono::{DateTime, Utc};

#[derive(Identifiable, Queryable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
}
