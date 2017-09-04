use super::*;

// Test that the default selected index is 0
#[test]
fn default_selected_is_zero() {
    let controller = Controller::new(&vec!["a", "b"].as_slice());
    assert_eq!(controller.selected_idx("a"), 0);
    assert_eq!(controller.selected_idx("b"), 0);
}

// Test that the selected index is retained while swapping views
#[test]
fn focus_works() {
    let mut controller = Controller::new(&vec!["a", "b"].as_slice());
    controller.set_selected_idx(1);
    controller.set_focus("b");
    controller.set_selected_idx(2);
    controller.set_focus("a");

    assert_eq!(controller.focused(), "a");
    assert_eq!(controller.selected_idx("a"), 1);
    assert_eq!(controller.selected_idx("b"), 2);
}

#[test]
fn set_safe() {
    let mut controller = Controller::new(&vec!["a"].as_slice());

    assert_eq!(controller.set_selected_idx_safe(3, 2), 2);
    assert_eq!(controller.selected_idx("a"), 2);

    assert_eq!(controller.set_selected_idx_safe(1, 2), 1);
    assert_eq!(controller.selected_idx("a"), 1);
}
