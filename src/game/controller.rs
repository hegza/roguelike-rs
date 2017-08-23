use rpglib::*;

pub struct Controller {
    pub inventory: usize,
    pub story_option: usize,
    pub equipment: ItemSlot,
    pub focus: usize, // Id of view
    // TODO: focus() -> &View
    pub max_views: usize,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            inventory: 0,
            story_option: 0,
            equipment: ItemSlot::MainHand,
            focus: 1,
            max_views: 4,
        }
    }
}
