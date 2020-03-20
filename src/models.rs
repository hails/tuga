use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Process {
    pub id: i32,
    pub code: String,
    pub telegram_user_id: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
