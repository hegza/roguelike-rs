use super::*;
use range::*;
use rpglib::*;
use rpglib::generator::*;
use game::controller::*;

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
    OpenTreasure {
        items: Vec<Item>
    },
    Final
}

impl StoryState {
    pub fn has_free_nav(&self) -> bool {
        use self::StoryState::*;
        match *self {
            CombatEncounter {..} => false,
            _ => true
        }
    }
    pub fn options(&self) -> Vec<String> {
        use self::StoryState::*;
        match *self {
            CombatEncounter {ref combat, ..} => {
                let mut strs: Vec<&str> = vec![];
                match combat.has_ended() {
                    false => {
                        strs.extend(&vec!["Attack", "Equip", "Unequip"]);
                    },
                    true => {
                        strs.extend(&vec!["Search"]);
                    }
                }
                strs.iter().map(|&s| s.to_owned()).collect()
            },
            OpenTreasure {ref items} => {
                let mut strs: Vec<String> = Vec::with_capacity(items.len() + 1);
                for item in items {
                    strs.push(format!("Pick up {}", item.name()))
                }
                strs.push("Travel east...".to_owned());
                strs
            },
            Final => {vec!["Yay..?".to_owned()]}
        }
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        let character = create_character();

        let dungeon = create_dungeon();

        let first_monster = {
            let room = dungeon.first_room();
            room.monster.as_ref().expect("first room in dungeon must have a monster").clone()
        };
        let combat = Combat::new(&character, &first_monster);
        let first_encounter = StoryState::CombatEncounter {
            monster: first_monster,
            combat
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
    let item_1 = equipment("long sword", 4, Slot::Hand, vec![])
        .damage(2)
        .build();
    let item_2 = equipment("stone", 3, Slot::Hand, vec![]).build();
    let item_3 = equipment("short sword", 2, Slot::Hand, vec![])
        .damage(1)
        .build();
    let item_4 = consumable("food ration", 1, vec![]).build();

    my_character.inventory.put(item_1.into());
    my_character.inventory.put(item_2.into());
    my_character.inventory.put(item_3.into());
    my_character.inventory.put(item_4.into());

    my_character
}

lazy_static! {
    // Zarad-dul, F, goddess of creating holes in people, trees, and the ground, etc; consumed with an all-encompassing rage
    // Iahu, M, god of pointy objects, perpetually lost (so don't pray to him unless you have a map) and befuddled
    // Eregek, F, goddess of strangulation, careless (and might strangle you accidentally, should you ask for help)
    // Gzolneb, M, god of death by crushing, deeply insecure and prone to overkilling everything ever
    // Urra, F, goddess of ducking and anxiety attacks, fidgety, twitchy, and high-strung

    static ref SEED: Vec<usize> = vec![1, 2, 3, 4];
    static ref MONSTER_POOL: Vec<Monster> = vec![
        MonsterBuilder::new("goblin", 1, 3)
                .difficulty(1).keywords(&["goblin"]).spawn(),
        MonsterBuilder::new("giant spider", 1, 3)
                .difficulty(2).keywords(&["giant", "spider"]).spawn(),
        MonsterBuilder::new("human thug", 1, 3)
                .difficulty(3).keywords(&["human", "eregek"]).spawn(),
        MonsterBuilder::new("crazed impaler", 1, 3)
                .difficulty(4).keywords(&["iahu"]).spawn(),
        MonsterBuilder::new("zombie-goblin", 1, 3)
                .difficulty(5).keywords(&["goblin", "undead"]).spawn(),
        MonsterBuilder::new("skulking cadaver", 1, 3)
                .difficulty(6).keywords(&["human", "undead"]).spawn(),
        MonsterBuilder::new("imp", 1, 3)
                .difficulty(7).keywords(&["demon", "urra"]).spawn(),
        MonsterBuilder::new("ogre mauler", 1, 3)
                .difficulty(8).keywords(&["giant", "gzolneb"]).spawn(),
        MonsterBuilder::new("demon", 1, 3)
                .difficulty(9).keywords(&["demon", "urra"]).spawn(),
        MonsterBuilder::new("lich", 1, 3)
                .difficulty(10).keywords(&["undead"]).spawn(),
    ];
    static ref THEME_KEYWORD_POOL: Vec<Keyword> = vec![
        // Gods
        "zarad-dul", "iahu", "eregek", "gzolneb", "urra",
        // Creature types
        "spider", "goblin", "demon", "undead", "human",
        // Magical effects
        "giant"]
        .iter().map(|x: &&str| Keyword::from(x.to_string().clone())).collect();
}

fn create_dungeon() -> Dungeon {
    let template_monsters = vec![];
    const DUNGEON_KEYWORD_COUNT: usize = 10;
    const ARCH_KEYWORD_COUNT: usize = 5;
    const AREA_KEYWORD_COUNT: usize = 3;
    const ARCH_COUNT: usize = 2;
    let num_areas_in_arch: Range = Range::new(2, 1);
    let num_main_rooms_in_area: Range = Range::new(3, 2);

    let g = Generator::new(MONSTER_POOL.as_slice(),template_monsters.as_slice(),
        THEME_KEYWORD_POOL.as_slice(), DUNGEON_KEYWORD_COUNT,
        ARCH_KEYWORD_COUNT, AREA_KEYWORD_COUNT, ARCH_COUNT, num_areas_in_arch,
        num_main_rooms_in_area
    );

    // Act
    let dungeon = g.generate(&SEED.as_slice());
    eprintln!("Dungeon:");
    for (i, room) in dungeon.rooms.iter().enumerate() {
        let s = "\tRoom ".to_string() +
            &format!("{} ({}): {:?}", i+1, room.keyword.id, room.monster.as_ref().unwrap());
        eprintln!("{}", s);
    }
    dungeon
}
