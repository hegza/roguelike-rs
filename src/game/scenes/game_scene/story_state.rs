use super::*;
use super::story_option::*;
use game::handle_input::*;
use game::handle_input::command::Command::*;

pub enum StoryState {
    /// Current encounter.
    Encounter(Option<CombatEncounter>),
    OpenTreasure { items: Vec<Item> },
    Final,
}

pub struct CombatEncounter {
    /// The monster that the player is currently fighting with.
    pub monster: Monster,
    /// The ongoing combat
    pub combat: Combat,
}

impl CombatEncounter {
    pub fn new(player: &Character, room: &Room) -> Option<CombatEncounter> {
        match room.monster {
            None => None,
            Some(ref m) => {
                let monster = m.clone();
                let combat = Combat::new(player, &monster);
                Some(CombatEncounter {
                    monster: monster,
                    combat: combat,
                })
            }
        }
    }
}

impl StoryState {
    pub fn options(&self) -> Vec<StoryOption> {
        use self::StoryState::*;
        use self::StoryOption::*;
        match *self {
            Encounter(ref encounter) => match *encounter {
                Some(ref encounter) => {
                    match encounter.combat.has_ended() {
                        false => vec![Attack],
                        // TODO: should check if player's dead instead of assuming that monster is
                        true => vec![Search],
                    }
                }
                None => vec![Search],
            },
            OpenTreasure { ref items } => {
                let mut options = Vec::with_capacity(items.len() + 2);
                for i in 0..items.len() {
                    options.push(PickUp(i));
                }
                options.push(GoEast);
                options
            }
            Final => vec![],
        }
    }
    pub fn has_free_nav(&self) -> bool {
        use self::StoryState::*;
        match *self {
            Encounter(ref combat_encounter) => {
                if combat_encounter.is_some() {
                    return false;
                }
                true
            }
            _ => true,
        }
    }
    /// Returns (keys, description)
    pub fn active_guides(&self) -> Vec<(String, String)> {
        let bindings: Vec<(&char, &Command)> = KEY_BINDINGS.iter().collect();

        let mut guides: Vec<(String, String)> = vec![];
        // Describe the nav controls
        if self.has_free_nav() {
            // Find nav keys
            guides.push(create_guide(
                "navigate windows",
                Box::new(|cmd| match *cmd {
                    Nav(..) => true,
                    _ => false,
                }),
                &bindings,
            ));
        }
        // Describe item selector
        {
            // Find the item selector keys
            guides.push(create_guide(
                "change selection",
                Box::new(|cmd| match *cmd {
                    MoveSelect(..) => true,
                    _ => false,
                }),
                &bindings,
            ));
        }
        // Describe confirm keys
        {
            guides.push(create_guide(
                "confirm selection",
                Box::new(|cmd| match *cmd {
                    Confirm => true,
                    _ => false,
                }),
                &bindings,
            ));
        }
        // Describe drop keys
        if let StoryState::OpenTreasure { .. } = *self {
            guides.push(create_guide(
                "drop item",
                Box::new(|cmd| match *cmd {
                    Drop => true,
                    _ => false,
                }),
                &bindings,
            ));
        }
        guides.sort_by(|a, b| a.1.cmp(&b.1));
        guides
    }
}

fn create_guide(
    guide_string: &str,
    filter: Box<Fn(&Command) -> bool>,
    all_bindings: &Vec<(&char, &Command)>,
) -> (String, String) {
    let bindings: Vec<(&char, &Command)> = all_bindings
        .iter()
        .filter(|kv| filter(kv.1))
        .map(|&kv| kv)
        .collect();
    let keys: Vec<String> = bindings.iter().map(|&(c, _)| c.to_string()).collect();
    (keys.join("/"), guide_string.to_owned())
}
