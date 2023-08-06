use crate::schema::*;

use chrono::{NaiveDateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Identifiable, Insertable, Queryable, Debug)]
pub struct Url {
    id: Uuid,
    url: String,
    shortened: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Url {
    pub fn new(id: Uuid, url: String, shortened: String) -> Self {
        Self {
            id,
            url,
            shortened,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    pub fn id(self) -> Uuid {
        self.id
    }

    pub fn shortened(self) -> String {
        self.shortened
    }
}

#[derive(Insertable, Debug)]
pub struct Visit {
    url_id: Uuid,
    count: i64,
}

impl Visit {
    pub fn new(url_id: Uuid) -> Self {
        Self { url_id, count: 0 }
    }
}
