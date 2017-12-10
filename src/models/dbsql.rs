use std::env;
use std::sync::Arc;
use sapper::Key;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use sapper::{SapperApp, SapperAppShell, Request, Response, Result, SapperModule};



pub struct AppDB;
impl Key for AppDB { type Value = Arc<PgConnection>; }



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
