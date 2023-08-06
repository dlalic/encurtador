use crate::config::app_url;
use crate::resources;

use diesel::PgConnection;
use gotham::hyper::header::CONTENT_TYPE;
use gotham::hyper::Method;
use gotham::router::builder::{self, DrawRoutes};
use gotham::router::Router;
use gotham_middleware_diesel::DieselMiddleware;
use gotham_restful::cors::{Headers, Origin};
use gotham_restful::gotham::middleware::logger::RequestLogger;
use gotham_restful::gotham::pipeline::{new_pipeline, single_pipeline};
use gotham_restful::*;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;

static API_URL: &str = "/api/v1";

fn api_router(repo: Repo) -> Router {
    let (chain, pipelines) = single_pipeline(
        new_pipeline()
            .add(DieselMiddleware::new(repo))
            .add(RequestLogger::new(log::Level::Info))
            .add(CorsConfig {
                origin: Origin::Star,
                headers: Headers::List(vec![CONTENT_TYPE]),
                max_age: 86400,
                credentials: false,
            })
            .build(),
    );
    builder::build_router(chain, pipelines, |route| {
        let info = OpenapiInfo {
            title: format!("{} API", env!("CARGO_PKG_NAME")),
            version: env!("CARGO_PKG_VERSION").to_string(),
            urls: vec![app_url() + API_URL],
        };
        route.with_openapi(info, |mut route| {
            route.resource::<resources::urls::Resource>("urls");
            route.openapi_spec("openapi");
            route.openapi_doc("api_doc");
        });
        for method in [Method::GET, Method::POST, Method::PUT, Method::DELETE] {
            route.cors("/urls", method.clone());
        }
    })
}

pub fn router(repo: Repo) -> Router {
    builder::build_simple_router(|route| {
        route.delegate(API_URL).to_router(api_router(repo.clone()));
    })
}
