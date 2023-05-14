use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::{reject::Reject};

#[derive(Debug, Serialize)]
struct QuestionId(String);

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title:{}, content:{}, tags:{:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}
impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}
// impl std::fmt::Debug for Question {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{:?}", self.tags)
//     }
// }
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "no Id provided1")),
        }
    }
}
// fn main() {
//     let q_id: QuestionId = QuestionId::from_str("10").expect("No Id Provided");
//     let tags: Vec<String> = vec![String::from("scrum")];
//     let q = Question::new(
//         q_id,
//         String::from("title"),
//         "content".to_string(),
//         Some(tags),
//     );
//     println!("{}", q);
// }
#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

pub async fn get_question() -> Result<impl warp::Reply, warp::Rejection> {
    let q_id: QuestionId = QuestionId::from_str("10").expect("No Id Provided");
    let tags: Vec<String> = vec![String::from("scrum")];
    let question = Question::new(
        q_id,
        String::from("title"),
        "content".to_string(),
        Some(tags),
    );
    let result = match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    };
    return result;
}
