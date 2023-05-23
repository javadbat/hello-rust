
// use questions::QuestionId;
use handle_errors::return_error;
use store::Store;
use warp::{
    http::Method,
    Filter,
};
// mod questions;
mod routes;
mod store;
mod types;
#[tokio::main]
async fn main() {
    run_server(1337).await;
}
async fn run_server(port: u16) {
    let my_store = Store::new();
    let store_filter = warp::any().map(move || my_store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
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
        .recover(return_error);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
// async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
//     if let Some(error) = r.find::<PaginationError>() {
//         Ok(warp::reply::with_status(
//             error.to_string(),
//             StatusCode::RANGE_NOT_SATISFIABLE,
//         ))
//     } else if let Some(error) = r.find::<QuestionError>() {
//         Ok(warp::reply::with_status(
//             error.to_string(),
//             StatusCode::NOT_FOUND,
//         ))
//     } else if let Some(error) = r.find::<CorsForbidden>() {
//         Ok(warp::reply::with_status(
//             error.to_string(),
//             StatusCode::FORBIDDEN,
//         ))
//     } else if let Some(error) = r.find::<BodyDeserializeError>() {
//         Ok(warp::reply::with_status(
//             error.to_string(),
//             StatusCode::UNPROCESSABLE_ENTITY,
//         ))
//     } else {
//         Ok(warp::reply::with_status(
//             "Route not found".to_string(),
//             StatusCode::NOT_FOUND,
//         ))
//     }
// }