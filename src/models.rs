use diesel::Queryable;

#[derive(serde::Serialize, Queryable)]
pub struct Baza {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}


