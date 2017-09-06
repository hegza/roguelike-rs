pub mod command;

mod inventory;
mod character;
mod story;

pub use self::command::*;
use game::*;
use game::scenes::*;
use self::story::*;
use std::collections::HashMap;
use game::scenes::StoryState::*;

use self::command::Command::*;
use self::command::GlobalCommand::*;

lazy_static!{
    pub static ref KEY_BINDINGS: HashMap<char, Command> = hashmap!(
        'q' => Global(Quit),
        'c' => Global(Cheat("combat_scene")),
        'e' => Confirm,
        'd' => Drop,
        'h' => Nav(Direction::Left),
        'l' => Nav(Direction::Right),
        'k' => MoveSelect(Direction::Up),
        'j' => MoveSelect(Direction::Down)
    );
}

impl From<char> for Command {
    fn from(c: char) -> Command {
        if !KEY_BINDINGS.contains_key(&c) {
            return Unknown;
        }
        KEY_BINDINGS[&c]
    }
}

/// This function applies input to the game state. Returns if execution should
/// continue;
pub fn apply(cmd: Command, state: &mut GameState) -> bool {
    // Handle global commands
    if let Command::Global(g_cmd) = cmd {
        match g_cmd {
            GlobalCommand::Quit => {
                return false;
            }
            c => {
                eprintln!("Warning: unhandled global command: '{:?}'", c);
            }
        }
    }

    match state.scene {
        Scene::Title => handle_input_for_title(cmd, state),
        Scene::Game(ref mut game) => handle_input_for_game(cmd, game),
    }

    return true;
}

fn handle_input_for_game(cmd: Command, game: &mut GameScene) {
    use self::Direction::*;

    // Navigate between widgets
    if game.story.has_free_nav() {
        if let Command::Nav(dir) = cmd {
            match dir {
                Right => {
                    game.controller.focus_next();
                }
                Left => {
                    game.controller.focus_prev();
                }
                _ => {}
            }
        }
    }

    // Delegate input handling to a specific handler based on focus
    let advance = match game.controller.focused() {
        "inventory" => Inventory::handle_input(cmd, game),
        "character" => Character::handle_input(cmd, game),
        "story" => Story::handle_input(cmd, game),
        _ => false,
    };
    if advance {
        match game.story {
            Encounter(ref mut encounter) => match *encounter {
                Some(ref mut encounter) => {
                    encounter
                        .combat
                        .apply_round(&mut game.player, &mut encounter.monster);
                }
                None => {}
            },
            _ => {}
        }
    }
}

fn handle_input_for_title(cmd: Command, state: &mut GameState) {
    if let Command::Confirm = cmd {
        state.scene = Scene::game();
    }
}

trait HandleInput {
    /// The input framework will call this function when a widget is focused.
    fn handle_input(cmd: Command, state: &mut GameScene) -> bool;
}
