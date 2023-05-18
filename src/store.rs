use crate::questions::{Question, QuestionId};
use std::{collections::HashMap};
#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}
impl Store {
    pub fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }
    // fn add_question(mut self, new_question: Question) -> Self {
    //     self.questions.insert(new_question.id.clone(), new_question);
    //     self;
    // }
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}