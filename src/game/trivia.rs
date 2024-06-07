use rand::{thread_rng, prelude::SliceRandom};
use serde::Deserialize;
use crate::util::escape_html;
use crate::constanst;

pub enum TriviaGameType {
    AnyType,
    Boolean,
    Multiple,
}

pub enum TriviaGameDifficulty {
    Any,
    Easy,
    Hard,
    Medium
}

pub enum TriviaGameCategory {
    AnyCategory = 0,
    GeneralKnowledge = 9,
    EntertainmentBooks = 10,
    EntertainmentFilm = 11,
    EntertainmentMusic = 12,
    EntertainmentMusicalAndTheatre = 13,
    EntertainmentTelevision = 14,
    EntertainmentVideoGames = 15,
    EntertainmentBoardGames = 16,
}

impl TriviaGameCategory {
    pub fn get_str<'a>() -> [&'a str; 9] {
        [
            "Any Category",
            "General Knowledge",
            "Entertainment Books",
            "Entertainment Film",
            "Entertainment Music",
            "Entertainment Musical And Theatre",
            "Entertainment Television",
            "Entertainment Video Games",
            "Entertainment Board Games",
        ]
    }

    pub fn select_by_str(index: &str) -> u8 {
        let category = TriviaGameCategory::get_str();
        let index = category.iter().position(|&x| x == index);
        if let Some(index) = index {
            match index {
                0 => 0,
                _ => (index + 7) as u8,
            }
        } else {
            panic!("wow youve entered wrong category");
        }
    }

    pub fn select_by_enum(index: TriviaGameCategory) -> u8 {
        index as u8
    }
}

impl TriviaGameType {
    pub fn get_str<'a>() -> [&'a str; 3] {
        [
            "Any Type",
            "Multiple Choice",
            "True or False",
        ]
    }

    pub fn select_by_str(index: &str) -> &'static str {
        match index {
            "Multiple Choice" => "multiple",
            "True or False"=> "boolean",
            _ => "any"
        }
    }

    pub fn select_by_enum(index: TriviaGameType) -> &'static str {
        match index {
            TriviaGameType::Boolean => "boolean",
            TriviaGameType::Multiple => "multiple",
            TriviaGameType::AnyType => "any",
        }
    }
}

impl TriviaGameDifficulty {
    pub fn get_str<'a>() -> [&'a str; 4] {
        [
            "Any Difficulty",
            "Easy",
            "Hard",
            "Medium",
        ]
    }

    pub fn select_by_str(index: &str) -> &'static str {
        match index {
            "Easy" => "easy",
            "Hard" => "hard",
            "Medium" => "medium",
            _ => "any"
        }
    }

    pub fn select_by_enum(index: TriviaGameDifficulty) -> &'static str {
        match index {
            TriviaGameDifficulty::Easy => "easy",
            TriviaGameDifficulty::Medium => "medium",
            TriviaGameDifficulty::Hard => "hard",
            TriviaGameDifficulty::Any => "any"
        }
    }
}

pub struct TriviaGameOptions {
    pub number_of_questions: u8,
    pub category: u8,
    pub difficulty: &'static str,
    pub r#type: &'static str,
    pub http_client: reqwest::Client,
}

pub struct TriviaGame {
    questions: Vec<OpenTDBResult>,
    question_index: u8,
    right_answers: u8,
    option: TriviaGameOptions,
}

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
        let mut result = 1;
        for _i in 0..5 {
            result = self.retrieve_questions().await?;
            if result == 0 { break; }
        }

        if result != 0 { panic!("Failed to retrieve_questions in 5 attempt")}
        Ok(())
    }

    pub fn get_question(&self) -> String {
        escape_html(&self.get_question_data().question)
    }

    pub fn get_difficulty(&self) -> String {
        escape_html(&self.get_question_data().difficulty)
    }

    pub fn get_category(&self) -> String {
        escape_html(&self.get_question_data().category)
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
        let mut query = Vec::<(&str, String)>::new();
        query.push(("amount", self.option.number_of_questions.to_string()));

        if self.option.category != 0 { query.push(("category", self.option.category.to_string())) }
        if self.option.r#type != "any" { query.push(("type", self.option.r#type.to_string()))}
        if self.option.difficulty != "any" { query.push(("difficulty", self.option.difficulty.to_string()))}

        let body = self.option.http_client
            .get(constanst::api_url::OPEN_TDB)
            .query(&query)
            .send()
            .await?
            .json::<OpenTDBResponse>()
            .await?;
        self.questions = body.results;

        Ok(body.response_code)
    }
}
