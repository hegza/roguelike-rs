use super::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;

impl Render for Game {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        // Create game info view
        let game_info = GameInfo::new(&self.character, &self.ticks);
        let story = Story { current_scene: &self.scene };
        let map = Map {};

        // Split the view in two horizontally
        Group::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &area, |t, chunks| {
                // Split the left view in two vertically
                Group::default()
                    .margin(0)
                    .direction(Direction::Vertical)
                    .sizes(&[Size::Percent(67), Size::Percent(33)])
                    .render(t, &chunks[0], |t, chunks| {
                        story.render(t, &chunks[0], &ctrl);
                        map.render(t, &chunks[1], &ctrl);
                    });
                // Split the right view in three vertically
                Group::default()
                    .margin(0)
                    .direction(Direction::Vertical)
                    .sizes(&[Size::Percent(50), Size::Percent(25), Size::Percent(25)])
                    .render(t, &chunks[1], |t, chunks| {
                        self.character.inventory.render(t, &chunks[0], &ctrl);
                        self.character.render(t, &chunks[1], &ctrl);
                        game_info.render(t, &chunks[2], &ctrl);
                    });

            });

    }
}
