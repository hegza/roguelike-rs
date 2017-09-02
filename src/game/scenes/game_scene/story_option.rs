use rpglib::*;

pub enum StoryOption {
    Attack,
    Equip,
    Unequip,
    Search,
    GoEast,
    PickUp(Item),
}

impl<'a> From<&'a StoryOption> for String {
    fn from(original: &'a StoryOption) -> Self {
        use self::StoryOption::*;
        match *original {
            Attack => "Attack".to_owned(),
            Equip => "Equip".to_owned(),
            Unequip => "Unequip".to_owned(),
            Search => "Search".to_owned(),
            GoEast => "Travel East...".to_owned(),
            PickUp(ref item) => format!("Pick up {}", item.name()),
        }
    }
}
