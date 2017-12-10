//use chrono::prelude::*;
use sapper::{Result, SapperModule, Request, Response, SapperRouter};
use sapper_std::{Context, render, PathParams, FormParams, QueryParams};

//use diesel;
//use diesel::prelude::*;
//use diesel::pg::PgConnection;



//use models::Articles as ArticlesModel;
//use models::{NewArticles,AppDB,establish_connection};

pub struct WebIndex;
impl WebIndex {
     fn index(_req: &mut Request) -> Result<Response> {
        
        res_html!("index.html", Context::new())
    }


}


impl SapperModule for WebIndex {
    
    fn before(&self, _req: &mut Request) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, _req: &Request, _res: &mut Response) -> Result<()> {
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        
        router.get("/",WebIndex::index);
        router.get("/index",WebIndex::index);
        
        
        Ok(())
        
    }
    
    
}
