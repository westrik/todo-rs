use super::schema::tasks;
use chrono::{DateTime, Utc};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
}
