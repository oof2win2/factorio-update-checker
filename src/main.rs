use dotenv::dotenv;
use gotham::hyper::Method;
use gotham::prelude::*;
use gotham::router::builder::*;
use gotham::router::Router;

mod handlers;

fn router() -> Router {
    build_simple_router(|route| {
        route
            .request(vec![Method::GET], "/versions/latest")
            .with_query_string_extractor::<handlers::versions::GetLatestParams>()
            .to_async_borrowing(handlers::versions::get_latest);
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    dotenv().ok();
    let addr = "127.0.0.1:3000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router()).unwrap();
}
