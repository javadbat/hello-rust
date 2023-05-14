use warp::{http::Method, http::StatusCode, Filter, Rejection, Reply};

mod questions;
#[tokio::main]
async fn main() {
    run_server(1337).await;
}
async fn run_server(port: u16) {
    // let hello = warp::get().map(|| format!("Hello, World!"));
    // let server_ip = warp::path("server-ip").map(async || {
    //     let result = get_my_ip().await;
    //     match result {
    //         Ok(_)=> {"call was successful"},
    //         _=>{"Error is accured"}
    //     }
    // });
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(questions::get_question)
        .recover(return_error);
    let routes = get_items.with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
async fn return_error(err: Rejection) -> Result<impl Reply, Rejection> {
    let cuase = err.find::<questions::InvalidId>();
    let is_invalid_id: bool = Option::is_some(&cuase);
    if is_invalid_id {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
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
