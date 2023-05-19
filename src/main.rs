use app_error::{PaginationError, QuestionError};
use questions::QuestionId;
use std::{collections::HashMap};
use store::Store;
use warp::{
    body::BodyDeserializeError, filters::cors::CorsForbidden, http::Method, http::StatusCode,
    Filter, Rejection, Reply,
};
mod app_error;
mod questions;
mod store;
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
        .and_then(get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);
    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_error);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, PaginationError> {
    if params.contains_key("start") && params.contains_key("end") {
        let pagination = Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(PaginationError::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(PaginationError::ParseError)?,
        };
        if pagination.start > pagination.end {
            Err(PaginationError::RangeError)
        } else {
            Ok(pagination)
        }
    } else {
        Err(PaginationError::MissingParameters)
    }
}
async fn get_question(
    params: HashMap<String, String>,
    my_store: store::Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let mut pagination: Pagination = extract_pagination(params)?;
        let res: Vec<questions::Question> =
            my_store.questions.read().await.values().cloned().collect();
        if res.len() <= pagination.start {
            return Err(warp::reject::custom(PaginationError::OutOfBound));
        }
        if res.len() < pagination.end {
            pagination.end = res.len()
        }
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<questions::Question> =
            my_store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}
async fn add_question(
    my_store: store::Store,
    question: questions::Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    my_store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
async fn update_question(
    id: String,
    my_store: store::Store,
    question: questions::Question,
) -> Result<impl Reply, Rejection> {
    match my_store
        .questions
        .write()
        .await
        .get_mut(&QuestionId::new(id))
    {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(QuestionError::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
async fn delete_question(id: String, my_store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match my_store.questions.write().await.remove(&QuestionId::new(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(QuestionError::QuestionNotFound)),
    }
}
async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<PaginationError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<QuestionError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
// async fn get_my_ip() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
