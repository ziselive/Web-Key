use chrono::prelude::*;
use sapper::{Result, SapperModule, Request, Response, SapperRouter};
use sapper_std::{Context, render, PathParams, FormParams, QueryParams};

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;


use AppDB;
use models::Articles as ArticlesModel;
use models::NewArticles;
use super::super::establish_connection;

pub struct AdminIndex;
impl AdminIndex {
     fn index(_req: &mut Request) -> Result<Response> {
        
        res_html!("admin/index.html", Context::new())
    }


}


impl SapperModule for AdminIndex {
    
    fn before(&self, _req: &mut Request) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, _req: &Request, _res: &mut Response) -> Result<()> {
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        
        router.get("/admin",AdminIndex::index);
        router.get("/admin/index",AdminIndex::index);
        
        
        Ok(())
        
    }
    
    
}
