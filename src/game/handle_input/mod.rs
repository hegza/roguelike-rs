pub mod command;

mod inventory;
mod character;

pub use self::command::*;
use game::*;
use game::render::story::*;

impl From<char> for Command {
    fn from(c: char) -> Command {
        use self::command::Command::*;
        use self::command::GlobalCommand::*;
        match c {
            'q' => Global(Quit),
            'c' => Global(Cheat("combat_scene")),
            'e' => Confirm,
            'h' => Nav(Direction::Left),
            'l' => Nav(Direction::Right),
            'k' => MoveSelect(Direction::Up),
            'j' => MoveSelect(Direction::Down),
            _ => Unknown,
        }
    }
}

/// This function applies input to the game state.
pub fn apply(cmd: Command, state: &mut GameState) -> bool {
    use self::Direction::*;

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

    // Navigate between widgets
    if let Command::Nav(dir) = cmd {
        match dir {
            Right => {
                state.controller.focus_next();
            }
            Left => {
                state.controller.focus_prev();
            }
            _ => {}
        }
    }

    // Delegate input handling to the widget itself
    match state.controller.focused() {
        "inventory" => {
            Inventory::handle_input(cmd, state);
        }
        "character" => {
            Character::handle_input(cmd, state);
        }
        "story" => {
            Story::handle_input(cmd, state);
        }
        _ => {}
    }

    return true;
}

trait HandleInput {
    /// The input framework will call this function when a widget is focused.
    fn handle_input(cmd: Command, state: &mut GameState);
}

impl HandleInput for Story {
    fn handle_input(cmd: Command, state: &mut GameState) {}
}
