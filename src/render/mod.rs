pub mod character;
pub mod inventory;
pub mod game_info;
pub mod game;
pub mod story;
pub mod map;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;
use game::*;
use view::View;
pub use self::inventory::*;
pub use self::game_info::*;
pub use self::character::*;
pub use self::story::*;
pub use self::map::*;
use rpglib::*;
use scenes::CombatScene;

pub trait Render {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller);
}

impl<'a> View for Story<'a> {
    fn id(&self) -> usize {
        1
    }
}

impl View for CombatScene {
    fn id(&self) -> usize {
        // Same as for Story
        1
    }
}

impl View for Inventory {
    fn id(&self) -> usize {
        2
    }
}

impl View for Character {
    fn id(&self) -> usize {
        3
    }
}

impl View for Map {
    fn id(&self) -> usize {
        0
    }
}
 