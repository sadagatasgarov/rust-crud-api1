use crate::models::NewBaza;
use crate::{models::Baza, schema::baza};
use diesel::prelude::*;
use diesel::{QueryResult, SqliteConnection};

pub struct BazaRepository;

impl BazaRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Baza> {
        baza::table.find(id).get_result::<Baza>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i32) -> QueryResult<Vec<Baza>> {
        baza::table.limit(limit.into()).load::<Baza>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_baza: NewBaza) -> QueryResult<Baza> {
        diesel::insert_into(baza::table)
            .values(new_baza)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;

        Self::find(c, last_id)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        baza::table.select(baza::id).order(baza::id.desc()).first(c)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, baz: Baza) -> QueryResult<Baza> {
        diesel::update(baza::table.find(id))
            .set((
                baza::email.eq(baz.email.to_owned()),
                baza::name.eq(baz.name.to_owned()),
            ))
            .execute(c)?;
        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(baza::table.find(id)).execute(c)
    }
}
