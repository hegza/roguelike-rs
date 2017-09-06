#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Copy, Clone)]
pub enum GlobalCommand {
    Quit,
    Cheat(&'static str),
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Global(GlobalCommand),
    Nav(Direction),
    MoveSelect(Direction),
    Drop,
    Confirm,
    Unknown,
}
