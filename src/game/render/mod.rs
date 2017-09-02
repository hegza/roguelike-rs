mod inventory;
mod character;
mod game_info;
mod map;
mod story;
mod title_screen;
mod help;

use game::*;
use tui::terminal::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::widgets::Widget;
use self::inventory::InventoryWidget;
use self::character::Character;
use self::game_info::GameInfo;
use self::map::Map;
use self::story::Story;
use self::title_screen::TitleScreen;
use self::help::Help;
use game::scenes::*;

pub struct UI {
    game_scene: ViewNode,
    title_screen: TitleScreen,
}

impl UI {
    pub fn new() -> UI {
        use self::ViewNode::*;
        use tui::layout::Direction::*;
        use tui::layout::Size::*;

        let story = Box::new(Display(Box::new(Story {})));
        let inventory = Box::new(Display(Box::new(InventoryWidget {})));
        let character = Box::new(Display(Box::new(Character {})));
        let game_info = Box::new(Display(Box::new(GameInfo {})));
        let map = Box::new(Display(Box::new(Map {})));
        let help = Box::new(Display(Box::new(Help {})));

        let game_scene = Split {
            direction: Horizontal,
            sizes: vec![Percent(50), Percent(50)],
            nodes: vec![
                Box::new(Split {
                    direction: Vertical,
                    sizes: vec![Percent(67), Percent(33)],
                    nodes: vec![story, map],
                }),
                Box::new(Split {
                    direction: Vertical,
                    sizes: vec![Percent(50), Percent(25), Percent(25)],
                    nodes: vec![
                        inventory, character,
                        Box::new(Split {
                            direction: Horizontal,
                            sizes: vec![Percent(50), Percent(50)],
                            nodes: vec![game_info, help]
                        })
                    ]
                }),
            ],
        };

        UI {
            game_scene,
            title_screen: TitleScreen::default(),
        }
    }
    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        match state.scene {
            Scene::Title => {
                render_title_screen(t, area, &self.title_screen);
            }
            Scene::Game(ref game) => {
                self.game_scene.render(t, area, game);
            }
        }
    }
}

trait GameView {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, scene: &GameScene);
}

enum ViewNode {
    Split {
        direction: Direction,
        sizes: Vec<Size>,
        nodes: Vec<Box<ViewNode>>,
    },
    Display(Box<GameView>),
    // TODO: SwapDisplay(),
}

impl ViewNode {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, scene: &GameScene) {
        use self::ViewNode::*;
        match *self {
            Split {
                ref direction,
                ref sizes,
                ref nodes,
            } => {
                Group::default()
                    .margin(0)
                    .direction(direction.clone())
                    .sizes(&sizes)
                    .render(t, area, |t, chunks| for i in 0..nodes.len() {
                        nodes[i].render(t, &chunks[i], scene);
                    });
            }
            Display(ref view) => {
                view.render(t, area, scene);
            }
        }
    }
}

fn render_title_screen(t: &mut Terminal<TermionBackend>, area: &Rect, title_screen: &TitleScreen) {
    title_screen.render(t, area);
}
