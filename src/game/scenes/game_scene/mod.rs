pub mod story_option;
pub mod story_state;

mod content;

pub use self::story_state::*;

use super::*;
use rpglib::*;
use game::controller::*;
use self::content::*;
use self::StoryState::*;

pub struct GameScene {
    pub controller: Controller,
    /// Generated dungeon for this session.
    pub dungeon: Dungeon,
    pub current_room: usize,
    /// Main character.
    pub player: Character,
    /// Story
    pub story: StoryState,
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
            // HACK: should use a sensible default value
            story: Final,
        };
        // Enter the first room before returning
        scene.enter_room(0);
        scene
    }
    pub fn enter_adjacent_room(&mut self, cp: CompassPoint) {
        match self.dungeon.get_adjacent(self.current_room, cp) {
            None => {
                self.story = Final;
            },
            Some(&room_id) => {
                self.enter_room(room_id);
            }
        }
    }
    fn enter_room(&mut self, room_id: usize) {
        eprintln!("player enters room {:?}", room_id);
        self.current_room = room_id;
        let room = self.dungeon.get_room(room_id);
        let monster = room.monster.as_ref().expect("room must have a monster").clone();
        let combat = Combat::new(&self.player, &monster);
        self.story = CombatEncounter {
            monster: monster,
            combat: combat,
        };
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
