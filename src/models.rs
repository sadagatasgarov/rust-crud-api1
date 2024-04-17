use super::schema::baza;

#[derive(serde::Serialize, Queryable)]
pub struct Baza {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}


#[derive(serde::Deserialize, Insertable)]
#[table_name = "baza"]
pub struct NewBaza {
    pub name: String,
    pub email: String,
}


