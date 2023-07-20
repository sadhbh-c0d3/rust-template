use std::io;

type Answer = (String, bool);

pub trait Smalltalk {
    fn answer(self) -> Answer;
}

impl Smalltalk for &str {
    fn answer(self) -> Answer {
        return match self {
            "Hi" => ("Hi, nice to meet you!".to_string(), false),
            "Hello" => ("Hello! Nice to see you again!".to_string(), false),
            "Bye" => ("Goodbye!".to_string(), true),
            _ => ("I didn't get that...".to_string(), false)
        };
    }
}

fn main() {
    println!("Say something!");

    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Oops! I've lost you!");

        let (answer_message, finish) = text.trim().answer();

        println!("{}", answer_message);

        if finish { break; }
    }
}
