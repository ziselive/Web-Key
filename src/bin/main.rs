#[macro_use] extern crate sapper_std;
extern crate sapper;
extern crate web_key;
extern crate env_logger;
extern crate dotenv;
use std::sync::Arc;
use dotenv::dotenv;
use sapper::{SapperApp, SapperAppShell, Request, Response, Result, SapperModule};
use web_key::{ArticlesModule,WebIndex,webarticle,AppDB,establish_connection,AdminArticles};




#[derive(Clone)]
struct MyApp;

impl SapperAppShell for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        sapper_std::init(req, None)?;

        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        sapper_std::finish(req, res)?;

        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    dotenv().ok();
    
    // let conn = Arc::new(establish_connection());
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(8080)
        // .init_global(Box::new(move |req: &mut Request| -> Result<()> {
        //     req.ext_mut().insert::<AppDB>(conn.clone());
            
        //     Ok(())
        // }))
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(ArticlesModule))
        .add_module(Box::new(WebIndex))
        .add_module(Box::new(AdminArticles));
    println!("Listening on http://127.0.0.1:8080");
    sapp.run_http();
    
}
//by sapper
