mod content;

use super::*;
use range::*;
use rpglib::*;
use rpglib::generator::*;
use game::controller::*;
use self::content::*;

pub struct GameScene {
    pub controller: Controller,
    /// Generated dungeon for this session.
    pub dungeon: Dungeon,
    /// Main character.
    pub player: Character,
    /// Story
    pub story: StoryState,
}

pub enum StoryState {
    /// Current encounter.
    CombatEncounter {
        /// The monster that the player is currently fighting with.
        monster: Monster,
        /// The ongoing combat
        combat: Combat,
    },
    OpenTreasure { items: Vec<Item> },
    Final,
}

impl StoryState {
    pub fn has_free_nav(&self) -> bool {
        use self::StoryState::*;
        match *self {
            CombatEncounter { .. } => false,
            _ => true,
        }
    }
    pub fn options(&self) -> Vec<String> {
        use self::StoryState::*;
        match *self {
            CombatEncounter { ref combat, .. } => {
                let mut strs: Vec<&str> = vec![];
                match combat.has_ended() {
                    false => {
                        strs.extend(&vec!["Attack", "Equip", "Unequip"]);
                    }
                    true => {
                        strs.extend(&vec!["Search"]);
                    }
                }
                strs.iter().map(|&s| s.to_owned()).collect()
            }
            OpenTreasure { ref items } => {
                let mut strs: Vec<String> = Vec::with_capacity(items.len() + 1);
                for item in items {
                    strs.push(format!("Pick up {}", item.name()))
                }
                strs.push("Travel east...".to_owned());
                strs
            }
            Final => vec!["Yay..?".to_owned()],
        }
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        let character = create_character();

        let dungeon = create_dungeon();

        let first_monster = {
            let room = dungeon.first_room();
            room.monster
                .as_ref()
                .expect("first room in dungeon must have a monster")
                .clone()
        };
        let combat = Combat::new(&character, &first_monster);
        let first_encounter = StoryState::CombatEncounter {
            monster: first_monster,
            combat,
        };

        GameScene {
            controller: Controller::new(&vec!["story", "inventory", "character"]),
            dungeon,
            player: character,
            story: first_encounter,
        }
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

fn create_character() -> Character {
    let mut attributes = CharacterAttributes::default();
    attributes.set(Attribute::Constitution, 8);
    let mut my_character = CharacterBuilder::new(2, 12, &attributes)
        .named("hegza")
        .build();

    for item in STARTING_ITEMS.iter() {
        my_character.inventory.put(item.clone().into());
    }

    my_character
}


fn create_dungeon() -> Dungeon {
    let template_monsters = vec![];
    const DUNGEON_KEYWORD_COUNT: usize = 10;
    const ARCH_KEYWORD_COUNT: usize = 5;
    const AREA_KEYWORD_COUNT: usize = 3;
    const ARCH_COUNT: usize = 2;
    let num_areas_in_arch: Range = Range::new(2, 1);
    let num_main_rooms_in_area: Range = Range::new(3, 2);

    let g = Generator::new(
        MONSTER_POOL.as_slice(),
        template_monsters.as_slice(),
        THEME_KEYWORD_POOL.as_slice(),
        DUNGEON_KEYWORD_COUNT,
        ARCH_KEYWORD_COUNT,
        AREA_KEYWORD_COUNT,
        ARCH_COUNT,
        num_areas_in_arch,
        num_main_rooms_in_area,
    );

    // Act
    let dungeon = g.generate(&SEED.as_slice());
    eprintln!("Dungeon:");
    for (i, room) in dungeon.rooms.iter().enumerate() {
        let s = "\tRoom ".to_string() +
            &format!(
                "{} ({}): {:?}",
                i + 1,
                room.keyword.id,
                room.monster.as_ref().unwrap()
            );
        eprintln!("{}", s);
    }
    dungeon
}
