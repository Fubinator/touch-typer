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
    time: std::time::Instant,
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
        let mut stdout = terminal.unwrap();

        for c in stdin.keys() {
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
                }
                termion::event::Key::Char(k) => {
                    self.input.push(k);

                    write!(stdout, "{}", k);
                }
                _ => break,
            }
            stdout.flush().unwrap();

            if self.input == self.words.join(" ") {
                break;
            }
        }
    }

    fn result(&self) {
        if self.input.trim() != self.words.join(" ") {
            println!("\nYou have mistyped somewhere. Please try again.");
        } else {
            println!("\nElapsed time: {:.2?}", self.time.elapsed());
            println!(
                "Words per minute: {:.2?}",
                10.0 / (self.time.elapsed().as_millis() as f64 / 60.0) * 1000.0
            );
        }
    }
}

fn main() {
    let mut game = Game {
        words: get_random_words(10),
        time: Instant::now(),
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
        "a".to_string(),
        "about".to_string(),
        "all".to_string(),
        "also".to_string(),
        "and".to_string(),
        "as".to_string(),
        "at".to_string(),
        "be".to_string(),
        "because".to_string(),
        "but".to_string(),
        "by".to_string(),
        "can".to_string(),
        "come".to_string(),
        "could".to_string(),
        "day".to_string(),
        "do".to_string(),
        "even".to_string(),
        "find".to_string(),
        "first".to_string(),
        "for".to_string(),
        "from".to_string(),
        "get".to_string(),
        "give".to_string(),
        "go".to_string(),
        "have".to_string(),
        "he".to_string(),
        "her".to_string(),
        "here".to_string(),
        "him".to_string(),
        "his".to_string(),
        "how".to_string(),
        "I".to_string(),
        "if".to_string(),
        "in".to_string(),
        "into".to_string(),
        "it".to_string(),
        "its".to_string(),
        "just".to_string(),
        "know".to_string(),
        "like".to_string(),
        "look".to_string(),
        "make".to_string(),
        "man".to_string(),
        "many".to_string(),
        "me".to_string(),
        "more".to_string(),
        "my".to_string(),
        "new".to_string(),
        "no".to_string(),
        "not".to_string(),
        "now".to_string(),
        "of".to_string(),
        "on".to_string(),
        "one".to_string(),
        "only".to_string(),
        "or".to_string(),
        "other".to_string(),
        "our".to_string(),
        "out".to_string(),
        "people".to_string(),
        "say".to_string(),
        "see".to_string(),
        "she".to_string(),
        "so".to_string(),
        "some".to_string(),
        "take".to_string(),
        "tell".to_string(),
        "than".to_string(),
        "that".to_string(),
        "the".to_string(),
        "their".to_string(),
        "them".to_string(),
        "then".to_string(),
        "there".to_string(),
        "these".to_string(),
        "they".to_string(),
        "thing".to_string(),
        "think".to_string(),
        "this".to_string(),
        "those".to_string(),
        "time".to_string(),
        "to".to_string(),
        "two".to_string(),
        "up".to_string(),
        "use".to_string(),
        "very".to_string(),
        "want".to_string(),
        "way".to_string(),
        "we".to_string(),
        "well".to_string(),
        "what".to_string(),
        "when".to_string(),
        "which".to_string(),
        "who".to_string(),
        "will".to_string(),
        "with".to_string(),
        "would".to_string(),
        "year".to_string(),
        "you".to_string(),
        "your".to_string(),
    ];

    let random_words: Vec<_> = words
        .choose_multiple(&mut rand::thread_rng(), amount)
        .cloned()
        .collect();

    return random_words;
}
