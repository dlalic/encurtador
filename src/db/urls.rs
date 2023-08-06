use crate::error::Error;
use crate::models::Url;
use crate::schema::urls;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn insert(url: Url, conn: &mut PgConnection) -> Result<String, Error> {
    diesel::insert_into(urls::dsl::urls)
        .values(url)
        .returning(urls::dsl::shortened)
        .get_result(conn)
        .map_err(Error::from)
}

pub fn select(url: String, conn: &mut PgConnection) -> Result<(Uuid, String), Error> {
    urls::dsl::urls
        .filter(urls::dsl::url.eq(url))
        .select((urls::dsl::id, urls::dsl::shortened))
        .get_result(conn)
        .map_err(Error::from)
}
