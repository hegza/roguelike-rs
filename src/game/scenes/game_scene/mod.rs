pub mod story_option;
pub mod story_state;

mod content;

pub use self::story_state::*;

use super::*;
use rpglib::*;
use game::controller::*;
use self::content::*;

pub struct GameScene {
    pub controller: Controller,
    /// Generated dungeon for this session.
    pub dungeon: Dungeon,
    /// Main character. Has inventory and equipment.
    pub player: Character,
    /// Story
    pub story: StoryState,
    current_room: usize,
}

impl GameScene {
    pub fn new() -> GameScene {
        let character = create_character();
        let dungeon = create_dungeon();

        let mut scene = GameScene {
            controller: Controller::new(&vec!["story", "inventory", "character"]),
            dungeon,
            current_room: 0,
            player: character,
            story: StoryState::OpenTreasure { items: vec![] },
        };
        // Enter the first room before returning
        scene.enter_room(0);
        scene
    }
    pub fn current_room(&self) -> &Room {
        self.dungeon.get_room(self.current_room)
    }
    pub fn enter_adjacent_room(&mut self, cp: CompassPoint) {
        match self.dungeon.get_adjacent(self.current_room, cp) {
            None => {
                self.story = StoryState::Final;
            }
            Some(room_id) => {
                self.enter_room(room_id);
            }
        }
    }
    fn enter_room(&mut self, room_id: usize) {
        eprintln!("player enters room {:?}", room_id);
        self.current_room = room_id;
        let room = self.dungeon.get_room(room_id);
        self.story = StoryState::Encounter(CombatEncounter::new(&self.player, room));
    }
}

impl<'a> TryFrom<&'a Scene> for &'a GameScene {
    type Err = &'static str;
    fn try_from(scene: &'a Scene) -> Result<Self, &'static str> {
        match *scene {
            Scene::Game(ref game) => Ok(game),
            _ => Err("unable to convert scene to game scene"),
        }
    }
}
