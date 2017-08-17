pub trait View {
    /// User provided unique identifier for this view. The identifier can be used to navigate using next and previous value.
    fn id(&self) -> usize;
}

#[derive(PartialEq, Eq)]
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
            _ => Command::Unknown,
        }
    }
}