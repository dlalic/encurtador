use diesel::PgConnection;
use encurtador::config::{app_address, database_url, load_and_validate_env_vars};
use encurtador::router::router;

use encurtador::db::migrations::run_migrations;
use env_logger::{Env, Target};
use futures::prelude::*;
use gotham_middleware_diesel::Repo;
use log::info;

#[tokio::main]
async fn main() {
    load_and_validate_env_vars();

    // By default it logs to stderr, using stdout instead
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .target(Target::Stdout)
        .init();

    let repo: Repo<PgConnection> = Repo::new(database_url().as_str());
    repo.run(|mut conn| run_migrations(&mut conn))
        .await
        .expect("Can not run migrations");

    let server = gotham::init_server(app_address(), router(repo.clone()));

    tokio::select! {
        _ = server.boxed() => { panic!("Server finished"); },
        _ = tokio::signal::ctrl_c() => { info!("Ctrl-C pressed"); },
    }

    info!("Shutting down gracefully");
}
