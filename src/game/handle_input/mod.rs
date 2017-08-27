pub mod command;

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

    // Delegate input handling to the widget itself
    return true;
}

trait HandleInput {
    /// The input framework will call this function when a widget is focused.
    fn handle_input(cmd: Command, state: &mut GameState);
}

impl HandleInput for Inventory {
    fn handle_input(cmd: Command, state: &mut GameState) {
        let idx = state.controller.selected_idx(&"inventory");
        let inventory = &state.character.inventory;
        if let Command::MoveSelect(dir) = cmd {
            match dir {
                Direction::Down => {
                    // Get bounds of item in current position
                    let (start, size) = inventory.bounds(idx as i32);
                    if start + size != inventory.capacity() {
                        // Move cursor below the current item
                        state.controller.set_selected_idx(start + size);
                    }
                }
                Direction::Up => {
                    // Get bounds of item in previous position
                    let (start, _) = inventory.bounds(idx as i32 - 1);
                    // Move cursor to the start of the item in previous position
                    state.controller.set_selected_idx(start);
                }
                _ => {}
            }
        }
    }
}

impl HandleInput for Character {
    fn handle_input(cmd: Command, state: &mut GameState) {
        use game::handle_input::command::Command::*;
        match cmd {
            MoveSelect(dir) => {
                let character = &state.character;
                let mut all_slots: Vec<&Slot> = character.slots();
                all_slots.sort();

                let cur_idx = state.controller.selected_idx("character");
                if dir == Direction::Down {
                    if cur_idx != all_slots.len() - 1 {
                        state.controller.set_selected_idx(cur_idx + 1);
                    }
                } else if dir == Direction::Up {
                    if cur_idx != 0 {
                        state.controller.set_selected_idx(cur_idx - 1);
                    }
                }
            }
            Confirm => {
                let character = &mut state.character;
                let idx = state.controller.selected_idx("character");
                let slot = (*character.nth_slot(idx).unwrap()).clone();
                if let Some(unequipped_item) = character.unequip(&slot) {
                    character.inventory.put(unequipped_item.into());
                }
            }
            _ => {}
        }
    }
}

impl HandleInput for Story {
    fn handle_input(cmd: Command, state: &mut GameState) {
    }
}
