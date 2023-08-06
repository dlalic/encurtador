use crate::db::urls;
use crate::error::Error;

use crate::db::visits::update_count;
use crate::models::Url;
use diesel::{Connection, PgConnection};
use gotham_derive::{StateData, StaticResponseExtender};
use gotham_restful::*;
use openapi_type::OpenapiType;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

const BASE_URL: &str = "https://tier.app";

#[derive(Resource)]
#[resource(create, search)]
pub struct Resource;

#[derive(Serialize, Deserialize, OpenapiType, Validate)]
pub struct CreateURL {
    #[validate(url)]
    pub url: String,
}

#[derive(Serialize, Deserialize, OpenapiType)]
struct URLResponse {
    shortened: String,
}

impl From<String> for URLResponse {
    fn from(shortened: String) -> Self {
        Self {
            shortened: [BASE_URL, &shortened].join("/"),
        }
    }
}

#[create]
fn create(body: CreateURL, conn: &mut PgConnection) -> Result<URLResponse, Error> {
    body.validate()?;
    let url = shorten(body.url);
    urls::insert(url, conn).map(URLResponse::from)
}

#[derive(Deserialize, StateData, StaticResponseExtender, OpenapiType, Clone)]
pub struct QueryStringExtractor {
    url: String,
}

#[search]
fn search(query: QueryStringExtractor, conn: &mut PgConnection) -> Result<URLResponse, Error> {
    conn.transaction::<_, Error, _>(|conn| {
        let (id, shortened) = urls::select(query.url, conn)?;
        update_count(id, conn)?;
        Ok(URLResponse::from(shortened))
    })
}

fn shorten(url: String) -> Url {
    let id = Uuid::new_v4();
    let b62 = base62::encode(id.as_u128());
    let shortened = b62.chars().take(7).collect();
    Url::new(id, url, shortened)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    use proptest_derive::Arbitrary;
    use std::collections::HashMap;
    use std::string::String;

    #[derive(Debug, Arbitrary)]
    struct TestURL {
        #[proptest(regex = "(http|https)")]
        protocol: String,
        #[proptest(regex = "[a-z0-9]+")]
        domain: String,
        #[proptest(regex = "[a-z]{3}")]
        domain_suffix: String,
        #[proptest(regex = "[a-z0-9]+")]
        path: String,
        #[proptest(regex = "[a-z0-9]+")]
        query: String,
    }

    impl TestURL {
        fn url(self) -> String {
            format!(
                "{}://{}.{}/{}?{}",
                self.protocol, self.domain, self.domain_suffix, self.path, self.query
            )
        }
    }

    #[test]
    fn url_uniqueness_on_proper_url() {
        let mut map = HashMap::new();
        let mut runner = TestRunner::default();
        let mut test_url = any::<TestURL>().new_tree(&mut runner).unwrap();
        while test_url.simplify() {
            let url = shorten(test_url.current().url());
            let shortened = url.shortened();
            assert_eq!(map.get(&shortened), None);
            map.insert(shortened, 0);
        }
    }

    #[test]
    fn url_uniqueness_on_any_characters() {
        let mut map = HashMap::new();
        let mut runner = TestRunner::default();
        let mut test_url = "\\PC*".new_tree(&mut runner).unwrap();
        while test_url.simplify() {
            let url = shorten(test_url.current());
            let shortened = url.shortened();
            assert_eq!(map.get(&shortened), None);
            map.insert(shortened, 0);
        }
    }

    #[test]
    fn uniqueness_on_same_url() {
        let mut map = HashMap::new();
        for _ in 0..50000 {
            let url = shorten("https://foo.com".to_string());
            let shortened = url.shortened();
            assert_eq!(map.get(&shortened), None);
            map.insert(shortened, 0);
        }
    }
}
