use tui::style::*;
use tui::widgets::*;
use tui::layout::*;
use tui::buffer::Buffer;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref PREFIXES: Vec<&'static str> = vec![
        // Negative, ~50 %
        "the Terrible", "the Horrible", "the Haunted", "the Forbidden",
        "the Profane", "the Obscene", "the Sinful", "the Bloody", "the Black",
        "the Deadly", "the Unfair", "the Eerie", "the Grim", "the Defiled",
        // Neutral, ~50 %
        "the Big", "the Strange", "the Unexplored", "the Silent",
        "the Enchanted", "the Unearthed", "the Ruined", "the Eternal",
        "the Divine", "the Unknown", "the Ancient", "the Unsanctioned",
        "the Forgotten",
        // Positive, 10 %
        "the Sacred", "the Hallowed", "the Consecrated",
        // Gods
        "Strolneg's", "Zarad-dul's", "Iahu's", "Eregek's", "Gzolneb's", "Urra's",
        // Trolls (< 5 %)
        "the Weird", "the Troll", "the Unlikely", "the Groovy"
    ];
    static ref NAMES: Vec<&'static str> = vec![
        "Dungeon", "Dungeon", "Dungeon",
        "Mines",
        "Cells",
        "Halls", "Halls",
        "Tunnel", "Tunnels",
        "Crypt", "Grotto",
        "Test",
        "Mountain",
        "Hills",
        "Feeding Grounds",
        "Caves",
        "Dwelling",
        "Temple", "Shrine", "Sanctuary", "Holy Place",
        "Burrows",
        "Vaults", "Reliquary",
        "Pyramid",
        "Courts",
        // Trolls (< 5 %)
        "Malls",
    ];
    static ref SUFFIXES: Vec<&'static str> = vec![
        "Gods", "Deities",
        "Monsters", "Monsters",
        "Spirits", "Ghosts",
        "Dead People",
        "Trolls",
        "Goblins",
        "Demons",
        "Spiders",
        "Giants",
        "Penitence", "Repentance", "Judgement", "Sacrifice",
        "Eternity",
        "Faith",
        "Hate",
        "Horrors", "Terrors", "Untold Terrors",
        "Undeath",
        "Monarchs",
        "Secrets",
        // Trolls (< 5 %)
        "Dates", "Deaf People", "the Infinite Dance", "Herwood", "Ducks"
    ];

    static ref ENDS: Vec<&'static str> = vec![
        ".", ".", ".", ".", ".", ".", ".", "!", "!", "!", "!", "..?"
    ];
}

pub struct TitleScreen {
    place: String,
}

impl TitleScreen {
    fn randomize(mut self) -> TitleScreen {
        let mut rng = thread_rng();
        self.place = format!(
            "{} {} of {}{}",
            rng.choose(&PREFIXES).unwrap(),
            rng.choose(&NAMES).unwrap(),
            rng.choose(&SUFFIXES).unwrap(),
            rng.choose(&ENDS).unwrap()
        );
        self
    }
}

impl Default for TitleScreen {
    fn default() -> TitleScreen {
        TitleScreen {
            place: String::new(),
        }.randomize()
    }
}

impl Widget for TitleScreen {
    fn draw(&self, area: &Rect, buf: &mut Buffer) {
        let title = format!("Welcome to {}", &self.place);
        let title_width = title.chars().count();

        let style = Style::default().fg(Color::Red).bg(Color::Gray);
        let title_x = (0.5 * area.width as f32 - 0.5 * title_width as f32) as u16;
        let title_y = (0.5 * area.height as f32) as u16 - 6;
        buf.set_string(title_x, title_y, &title, &style);

        let guide = "You may enter. (Press the 'e' -key.)";
        let guide_width = guide.chars().count();

        let guide_x = (0.5 * area.width as f32 - 0.5 * guide_width as f32) as u16;
        let guide_y = title_y + 6;
        buf.set_string(guide_x, guide_y, &guide, &style);
    }
}
