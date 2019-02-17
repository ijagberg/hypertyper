use rand::prelude::*;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::{thread, time};

struct Config {
    difficulty: usize,
}

fn main() {
    // Handle command line stuff
    let args: Vec<_> = env::args().collect();
    let difficulty = if args.len() > 1 {
        args[1].parse::<usize>().expect("Couldn't parse integer!")
    } else {
        0
    };

    let config = Config { difficulty };

    start_game(&config);
}

fn start_game(config: &Config) {
    // Get wordlist from file and split into vector
    let contents = fs::read_to_string("wordlist.txt").expect("Could not read file!");
    let words = match get_words(&contents, &config) {
        Ok(words) => words,
        _error => {
            eprintln!("Could not get words from wordlist.txt");
            return;
        }
    };

    // Print welcoming message and countdown
    print_countdown().expect("Could not print countdown message");

    match run(words) {
        Ok(_) => return,
        _error => {
            eprintln!("Some error occurred!");
            return;
        }
    }
}

fn get_words<'a>(contents: &'a String, config: &Config) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut rng = rand::thread_rng();

    for line in contents.lines() {
        if config.difficulty <= 0 || line.len() <= config.difficulty {
            words.push(line);
        }
    }
    words.shuffle(&mut rng);

    Ok(words)
}

fn print_countdown() -> Result<(), Box<Error>> {
    println!("Welcome to HyperTyper!");

    let mut seconds_left = 3;
    while seconds_left > 0 {
        print!("\rStarting in... {}", seconds_left);
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_secs(1));
        seconds_left -= 1;
    }
    println!("\rGo!                      ");
    thread::sleep(time::Duration::from_secs(1));

    Ok(())
}

fn run(words: Vec<&str>) -> Result<(), io::Error> {
    let mut written_words = 0;
    let timer = time::Instant::now();
    let mut user_input = String::new();

    // Add the first three words
    let mut display_words: VecDeque<&str> = VecDeque::new();
    for i in 0..=2 {
        display_words.push_back(words[i]);
    }
    let mut next_word_index = 3;

    while written_words < 30 && next_word_index < words.len() - 1 {
        println!(
            "{} ::: {} ::: {}",
            display_words[0], display_words[1], display_words[2]
        );
        user_input.clear();
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();

        match display_words.front() {
            Some(front_string) => {
                if front_string.eq(&user_input) {
                    written_words += 1;
                    display_words.pop_front();
                    display_words.push_back(words[next_word_index]);
                    next_word_index += 1;
                }
            }
            None => {}
        }
    }
    let elapsed = timer.elapsed();
    println!("Time: {}.{}s", elapsed.as_secs(), elapsed.subsec_millis());

    Ok(())
}
