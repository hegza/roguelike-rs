pub enum StoryOption {
    Attack,
    Equip,
    Unequip,
    Search,
    GoEast,
    PickUp(usize),
}

impl<'a> From<&'a StoryOption> for String {
    fn from(original: &'a StoryOption) -> Self {
        use self::StoryOption::*;
        match *original {
            Attack => "Attack".to_owned(),
            Equip => "Equip..".to_owned(),
            Unequip => "Unequip..".to_owned(),
            Search => "Search".to_owned(),
            PickUp(item_idx) => format!("Pick up {}", item_idx),
            GoEast => "Travel East...".to_owned(),
        }
    }
}

impl From<StoryOption> for String {
    fn from(original: StoryOption) -> Self {
        (&original).into()
    }
}
