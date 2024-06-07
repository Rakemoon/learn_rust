use std::{error::Error, io};
use learn_rust::game::trivia::{TriviaGame, TriviaGameCategory, TriviaGameOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let option = TriviaGameOptions {
        number_of_questions: 2,
        r#type: "any",
        difficulty: "easy",
        category: TriviaGameCategory::select_by_enum(TriviaGameCategory::EntertainmentVideoGames),
        http_client: reqwest::Client::new(),
    };

    let mut trivia = TriviaGame::new(option);

    println!("Preparing question.....");
    trivia.start().await?;

    println!("");

    while !trivia.is_end() {
        let question = trivia.get_question();
        let selection = trivia.get_selection();
        let limiter = String::from("=").repeat(question.len());

        println!("{limiter}");
        println!("{}", question);
        println!("{limiter}");
        for i in 0..selection.len() {
            let s = &selection[i];
            println!("{}. {}", i + 1, s);
        }
        println!("{limiter}");
        let mut answers = String::new();
        io::stdin()
            .read_line(&mut answers)
            .expect("Can't read line");
        let answers = answers.trim().parse::<usize>().unwrap_or(0);
        let answers = selection[answers - 1].to_owned();
        trivia.answer(&answers);
    }

    println!("{}", trivia.get_score());
    Ok(())
}
