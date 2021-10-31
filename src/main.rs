use rand::seq::SliceRandom;
use std::io;
use std::io::Write;
use std::time::Instant;
use termion;
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Game {
    words: Vec<String>,
    time: Option<std::time::Duration>,
    input: String,
}

impl Game {
    fn run(&mut self) {
        self.intro();

        self.game_loop();

        self.result()
    }

    fn intro(&self) {
        println!("Type these words correctly as fast you can.");
        println!(
            "{}\n{}\n{}",
            color::Fg(color::Blue),
            self.words.join(" "),
            color::Fg(color::White)
        );
        println!("Begin typing to start...\n");
    }

    fn game_loop(&mut self) {
        let stdin = io::stdin();
        let terminal = io::stdout().into_raw_mode();
        let mut is_running = false;
        let mut stdout = terminal.unwrap();
        let mut time: Option<Instant> = None;

        for c in stdin.keys() {
            if !is_running {
                is_running = true;
                time = Some(Instant::now());
            }

            match c.unwrap() {
                termion::event::Key::Backspace => {
                    let (x, y) = stdout.cursor_pos().unwrap();
                    write!(
                        stdout,
                        "{}{}{}",
                        termion::cursor::Goto(x - 1, y),
                        " ",
                        termion::cursor::Goto(x - 1, y)
                    );
                    self.input.pop();
                }
                termion::event::Key::Char(k) => {
                    self.input.push(k);

                    write!(stdout, "{}", k);
                }
                _ => break,
            }
            stdout.flush().unwrap();

            if self.input == self.words.join(" ") {
                match time {
                    Some(timer) => self.time = Some(timer.elapsed()),
                    None => {}
                }
                break;
            }
        }
    }

    fn result(&self) {
        if self.input.trim() != self.words.join(" ") {
            println!("\nYou have mistyped somewhere. Please try again.");
        } else {
            match self.time {
                Some(timer) => {
                    println!("\nElapsed time: {:.2?}", timer);
                    println!(
                        "Words per minute: {:.2?}",
                        10.0 / (timer.as_millis() as f64 / 60.0) * 1000.0
                    );
                }
                None => {}
            }
        }
    }
}

fn main() {
    let mut game = Game {
        words: get_random_words(10),
        time: None,
        input: String::new(),
    };

    println!("{}", color::Fg(color::Blue));
    println!("{}", get_intro_screen());
    println!("{}", color::Fg(color::White));

    game.run()
}

fn get_intro_screen() -> String {
    return String::from(
        "  ______                 __       ______                     
 /_  __/___  __  _______/ /_     /_  __/_  ______  ___  _____
  / / / __ \\/ / / / ___/ __ \\     / / / / / / __ \\/ _ \\/ ___/
 / / / /_/ / /_/ / /__/ / / /    / / / /_/ / /_/ /  __/ /    
/_/  \\____/\\__,_/\\___/_/ /_/    /_/  \\__, / .___/\\___/_/     
                                    /____/_/                ",
    );
}

fn get_random_words(amount: usize) -> Vec<String> {
    let words: Vec<String> = vec![
        "a", "about", "all", "also", "and", "as", "at", "be", "because", "but", "by", "can",
        "come", "could", "day", "do", "even", "find", "first", "for", "from", "get", "give", "go",
        "have", "he", "her", "here", "him", "his", "how", "I", "if", "in", "into", "it", "its",
        "just", "know", "like", "look", "make", "man", "many", "me", "more", "my", "new", "no",
        "not", "now", "of", "on", "one", "only", "or", "other", "our", "out", "people", "say",
        "see", "she", "so", "some", "take", "tell", "than", "that", "the", "their", "them", "then",
        "there", "these", "they", "thing", "think", "this", "those", "time", "to", "two", "up",
        "use", "very", "want", "way", "we", "well", "what", "when", "which", "who", "will", "with",
        "would", "year", "you", "your",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let random_words: Vec<_> = words
        .choose_multiple(&mut rand::thread_rng(), amount)
        .cloned()
        .collect();

    return random_words;
}
