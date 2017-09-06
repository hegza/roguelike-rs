#[cfg(test)]
mod tests;

use std::cmp::{max, min};

/// Allows tracking multiple windows each with it's own focused item.
pub struct Controller {
    focus: usize, // Idx of focused view in views
    views: Vec<View>,
}

impl Controller {
    pub fn new<'a>(view_ids: &'a [&'static str]) -> Controller {
        let views: Vec<View> = view_ids.iter().map(|s| View::new(*s)).collect();
        Controller {
            focus: 0,
            views: views,
        }
    }
    pub fn set_focus(&mut self, view_id: &'static str) {
        self.focus = self.views
            .iter()
            .position(|v| v.view_id == view_id)
            .expect("attempted to reference a view that has not been registered");
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
        self.views
            .iter()
            .find(|x| x.view_id == view_id)
            .unwrap()
            .selected_idx
    }
    pub fn selected_idx_safe(&self, view_id: &'static str, max_val: usize) -> usize {
        max(
            min(
                self.views
                    .iter()
                    .find(|x| x.view_id == view_id)
                    .unwrap()
                    .selected_idx,
                max_val,
            ),
            0,
        )
    }
    pub fn set_selected_idx(&mut self, idx: usize) {
        self.views[self.focus].selected_idx = idx;
    }
    pub fn focused(&self) -> &str {
        self.views.iter().nth(self.focus).unwrap().view_id
    }
    pub fn set_selected_idx_safe(&mut self, idx: i32, max: usize) -> usize {
        self.views[self.focus].set_safe(idx, max)
    }
}

/// Tracks the control state of a single view.
struct View {
    view_id: &'static str,
    selected_idx: usize,
}

impl View {
    fn new(view_id: &'static str) -> View {
        View {
            view_id,
            selected_idx: 0,
        }
    }
    pub fn set_safe(&mut self, val: i32, max_val: usize) -> usize {
        let nval = max(min(val, max_val as i32), 0) as usize;
        self.selected_idx = nval as usize;
        nval as usize
    }
}
