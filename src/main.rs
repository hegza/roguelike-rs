extern crate gag;
extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate range;
extern crate rpglib;
extern crate textwrap;
extern crate try_from;
extern crate tui;

mod game;

use game::GameState;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::io::Read;
use std::fs::{File, OpenOptions};
use gag::Redirect;

/// Rapid prototyping mode
static RP: bool = true;

fn get_char() -> char {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char)
        .unwrap()
}

fn redirect_stderr(filename: &'static str) -> Redirect<File> {
    // Open a log
    let log = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();

    Redirect::stderr(log).unwrap()
}

fn main() {
    // stderr is redirected to a file while this variable is in scope
    let _redirect_stderr = redirect_stderr("./errors.log");

    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut input: char;
    let mut game = GameState::new();
    loop {
        let size = terminal.size().unwrap();
        game.render(&mut terminal, &size);
        terminal.draw().unwrap();

        input = get_char();
        if !game.input(input) {
            break;
        }
    }

    terminal.show_cursor().unwrap();
}
