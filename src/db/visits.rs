use crate::error::Error;
use crate::models::Visit;
use crate::schema::visits;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn update_count(url_id: Uuid, conn: &mut PgConnection) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|conn| {
        diesel::insert_into(visits::dsl::visits)
            .values(Visit::new(url_id))
            .on_conflict_do_nothing()
            .execute(conn)
            .map_err(Error::from)?;
        diesel::update(visits::dsl::visits.filter(visits::dsl::url_id.eq(url_id)))
            .set(visits::dsl::count.eq(visits::dsl::count + 1))
            .execute(conn)
            .map_err(Error::from)
            .map(|_| ())
    })
}
