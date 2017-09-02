pub mod game_scene;

use try_from::*;
pub use self::game_scene::*;

pub enum Scene {
    /// Initial state
    Title,
    Game(GameScene),
}

impl Scene {
    pub fn game() -> Scene {
        Scene::Game(GameScene::new())
    }
}
