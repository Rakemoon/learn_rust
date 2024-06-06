use rand::{thread_rng, prelude::SliceRandom};
use serde::Deserialize;
use crate::util::escape_html;

pub struct TriviaGameOptions {
    pub number_of_questions: u8,
    pub category: u8,
    pub difficulty: &'static str,
    pub r#type: &'static str,
}

pub struct TriviaGame {
    questions: Vec<OpenTDBResult>,
    question_index: u8,
    right_answers: u8,
    option: TriviaGameOptions,
}


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OpenTDBResponse {
    response_code: u16,
    results: Vec<OpenTDBResult>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OpenTDBResult {
    r#type: String,
    difficulty: String,
    category: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

impl TriviaGame {
    pub fn new(option: TriviaGameOptions) -> TriviaGame {
        TriviaGame {
            questions: vec![],
            question_index: 0,
            right_answers: 0,
            option,
        }
    }

    pub async fn start(&mut self) -> Result<(), reqwest::Error> {
        loop {
            let result = self.retrieve_questions().await?;
            if result == 0 { break; }
        }
        Ok(())
    }

    pub fn get_question(&self) -> String {
        escape_html(&self.get_question_data().question)
    }

    pub fn get_selection(&self) -> Vec<String> {
        let mut selections = Vec::<String>::new();
        let question_data = self.get_question_data();
        if question_data.r#type == "boolean" { return vec!["True".into(), "False".into()] }
        question_data
            .incorrect_answers
            .iter()
            .for_each(|x| { selections.push(escape_html(x)) });
        selections.push(self.get_question_data().correct_answer.clone());
        let mut thread = thread_rng();
        selections.shuffle(&mut thread);
        selections
    }

    pub fn answer(&mut self, answer: &str) -> bool {
        let is_right = &self.get_question_data().correct_answer == answer;
        if is_right { self.right_answers += 1 }
        self.question_index += 1;
        is_right
    }

    pub fn is_end(&self) -> bool {
        (self.question_index as usize) >= self.questions.len()
    }

    pub fn get_score(&self) -> String {
        format!("{}/{}", self.right_answers, self.questions.len())
    }

    fn get_question_data(&self) -> &OpenTDBResult {
        &self.questions[self.question_index as usize]
    }

    async fn retrieve_questions(&mut self) -> Result<u16, reqwest::Error> {
        let mut url = String::from("https://opentdb.com/api.php?");
        url.push_str(&format!("amount={}", self.option.number_of_questions));
        //url.push_str(&format!("&category={}", self.option.category));
        //url.push_str(&format!("&difficulty={}", self.option.difficulty));
        //url.push_str(&format!("&type={}", self.option.r#type));

        let body = reqwest::get(url)
            .await?
            .json::<OpenTDBResponse>()
            .await?;
        self.questions = body.results;

        Ok(body.response_code)
    }
}
