use rpglib::*;

lazy_static! {
    // Zarad-dul, F, goddess of creating holes in people, trees, and the ground, etc; consumed with an all-encompassing rage
    // Iahu, M, god of pointy objects, perpetually lost (so don't pray to him unless you have a map) and befuddled
    // Eregek, F, goddess of strangulation, careless (and might strangle you accidentally, should you ask for help)
    // Gzolneb, M, god of death by crushing, deeply insecure and prone to overkilling everything ever
    // Urra, F, goddess of ducking and anxiety attacks, fidgety, twitchy, and high-strung

    pub static ref SEED: Vec<usize> = vec![1, 2, 3, 4];
    pub static ref MONSTER_POOL: Vec<Monster> = vec![
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
    pub static ref THEME_KEYWORD_POOL: Vec<Keyword> = vec![
        // Gods
        "zarad-dul", "iahu", "eregek", "gzolneb", "urra",
        // Creature types
        "spider", "goblin", "demon", "undead", "human",
        // Magical effects
        "giant"]
        .iter().map(|x: &&str| Keyword::from(x.to_string().clone())).collect();

    pub static ref STARTING_ITEMS: Vec<Item> = vec![
        equipment("long sword", 4, Slot::Hand, vec![])
            .damage(2)
            .build()
            .into(),
        equipment("stone", 3, Slot::Hand, vec![]).build().into(),
        equipment("short sword", 2, Slot::Hand, vec![])
            .damage(1)
            .build()
            .into(),
        consumable("food ration", 1, vec![]).build().into()
    ];
}
