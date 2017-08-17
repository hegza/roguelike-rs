extern crate rpglib;
extern crate tui;

mod game;
mod render;
mod ui;

use game::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::io::Read;

fn get_char() -> char {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char)
        .unwrap()
}

fn main() {
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut input: char;
    let mut game = Game::new();
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
