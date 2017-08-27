extern crate rpglib;
extern crate tui;
extern crate gag;
extern crate textwrap;

mod game;

use game::GameState;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use gag::Redirect;

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
    let print_redirect = redirect_stderr("./errors.log");

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
