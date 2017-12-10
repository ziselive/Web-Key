use chrono::prelude::*;
use sapper::{Result, SapperModule, Request, Response, SapperRouter};
use sapper_std::{Context, render, PathParams, FormParams, QueryParams};

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Clone)]
pub struct ArticlesModule;


use models::Articles as ArticlesModel;
use models::{NewArticles,AppDB,establish_connection};



macro_rules! get_db {
    ($req:expr, $dbkey:ty) => ({
        establish_connection()
    })
}



impl ArticlesModule {

    
    fn post_view(req: &mut Request) -> Result<Response> {
        use schema::webarticle::dsl::*;
        
        let db = get_db!(req, AppDB);

        let params = get_path_params!(req);
        let postid = t_param!(params, "postid").parse::<i64>().unwrap();
       
        let post = webarticle.find(postid)
                .first::<ArticlesModel>(&db)
                .expect("Error loading blog");
        
        let mut c = Context::new();
        c.add("post", &post);
        
        res_html!("post.html", c)
    }
    
    fn posts_view(_req: &mut Request) -> Result<Response> {
        use schema::webarticle::dsl::*;
        
        let db = get_db!(req, AppDB);
        let results = webarticle.load::<ArticlesModel>(&db).expect("Error loading blogs");
        
        let posts = results;
        
        println!("{:?}", posts);
        
        let mut c = Context::new();
        c.add("posts", &posts);
        
        res_html!("posts.html", c)
    }
 
}
// set before, after middleware, and add routers
impl SapperModule for ArticlesModule {
    
    fn before(&self, _req: &mut Request) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, _req: &Request, _res: &mut Response) -> Result<()> {
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        
       // router.get("/", ArticlesModule::index);
        router.get("/post/:postid", ArticlesModule::post_view);
        router.get("/posts", ArticlesModule::posts_view);
       // router.get("/post/create", ArticlesModule::create_post_view);
       // router.post("/post/create", ArticlesModule::create_post);
       // router.get("/post/:postid/edit", ArticlesModule::edit_post_view);
       // router.post("/post/edit", ArticlesModule::edit_post);
       // router.get("/post/:postid/delete", ArticlesModule::delete_post);
        
        Ok(())
        
    }
    
    
}
