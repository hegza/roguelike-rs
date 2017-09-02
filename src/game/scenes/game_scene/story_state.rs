use super::*;
use super::story_option::*;
use game::handle_input::*;
use game::handle_input::command::Command::*;

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
    pub fn options(&self) -> Vec<StoryOption> {
        use self::StoryState::*;
        use self::StoryOption::*;
        match *self {
            CombatEncounter { ref combat, .. } => match combat.has_ended() {
                false => vec![Attack, Equip, Unequip],
                true => vec![Search],
            },
            OpenTreasure { ref items } => {
                let mut options = Vec::with_capacity(items.len() + 1);
                options.extend(
                    items
                        .iter()
                        .map(|item| PickUp(item.clone()))
                        .collect::<Vec<StoryOption>>(),
                );
                options.push(GoEast);
                options
            }
            Final => vec![],
        }
    }
    pub fn has_free_nav(&self) -> bool {
        use self::StoryState::*;
        match *self {
            CombatEncounter { .. } => false,
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
