pub mod character;
pub mod inventory;
pub mod game_info;
pub mod game;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;
use game::*;
pub use self::inventory::*;
pub use self::game_info::*;
pub use self::character::*;

pub trait Render {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller);
}
