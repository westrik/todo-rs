use super::schema::tasks;
use chrono::{DateTime, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};

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

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Task", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("created_at", &self.created_at)?;
        s.serialize_field("updated_at", &self.updated_at)?;
        s.serialize_field("resolved_at", &self.resolved_at)?;
        s.end()
    }
}
