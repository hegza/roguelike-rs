
pub struct Controller {
    focus: usize, // Id of view
    views: Vec<(&'static str, usize)>,
}

impl Controller {
    pub fn new<'a>(view_names: &'a [&'static str]) -> Controller {
        let views: Vec<(&str, usize)> = view_names.iter().map(|s| (*s, 0)).collect();
        Controller {
            focus: 0,
            views: views,
        }
    }
    pub fn focus_next(&mut self) {
        if self.focus != self.views.len() - 1 {
            self.focus += 1;
        }
    }
    pub fn focus_prev(&mut self) {
        if self.focus != 0 {
            self.focus -= 1;
        }
    }
    pub fn selected_idx(&self, view_id: &'static str) -> usize {
        self.views.iter().find(|x| x.0 == view_id).unwrap().1
    }
    pub fn set_selected_idx(&mut self, idx: usize) {
        self.views[self.focus].1 = idx;
    }
    pub fn focused(&self) -> &str {
        self.views.iter().nth(self.focus).unwrap().0
    }
}
