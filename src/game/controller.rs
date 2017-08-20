use rpglib::*;

pub struct Controller {
    pub inventory: usize,
    pub equipment: ItemSlot,
    pub focus: usize, // Id of view
    // TODO: focus() -> &View
    pub max_views: usize,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            inventory: 0,
            equipment: ItemSlot::MainHand,
            focus: 0,
            max_views: 2,
        }
    }
}
