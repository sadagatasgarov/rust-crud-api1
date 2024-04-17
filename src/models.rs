use super::schema::baza;

#[derive(serde::Serialize, serde::Deserialize, Queryable)]
pub struct Baza {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}


#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = baza)]  //#[table_name = "baza"] deprecated
pub struct NewBaza {
    pub name: String,
    pub email: String,
}


