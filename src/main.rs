// use questions::QuestionId;
use dotenv::dotenv;
use handle_errors::return_error;
use store::Store;
use warp::{http::Method, Filter};
// mod questions;
mod routes;
mod store;
mod types;
#[tokio::main]
async fn main() {
    dotenv().ok();
    init_log();
    run_server(1337).await;
}
fn init_log(){
    let res = log4rs::init_file("log4rs.yaml", Default::default());
    match res {
        Err(err)=>{println!("{:?}", err)},
        _=>{}
    }
    log::error!("This is an error!");
    log::info!("This is info!");
    log::warn!("This is a warning!");
}
async fn run_server(port: u16) {
    
    let my_store = Store::new();
    let store_filter = warp::any().map(move || my_store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        log::info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
            );
       });
       let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::question::get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);
    let add_answer = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);
    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .with(log)
        .recover(return_error);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
