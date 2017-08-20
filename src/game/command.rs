
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub enum Command {
    Quit,
    Nav(Direction),
    MoveSelect(Direction),
    Confirm,
    Cheat(&'static str),
    Unknown,
}

impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            'q' => Command::Quit,
            'e' => Command::Confirm,
            'h' => Command::Nav(Direction::Left),
            'l' => Command::Nav(Direction::Right),
            'k' => Command::MoveSelect(Direction::Up),
            'j' => Command::MoveSelect(Direction::Down),
            'c' => Command::Cheat("combat_scene"),
            _ => Command::Unknown,
        }
    }
}
