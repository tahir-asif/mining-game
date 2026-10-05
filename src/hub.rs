use crate::{game::TransitionState, save::Profile};
use macroquad::{color::*, ui::root_ui, window::clear_background};

pub fn hub_update(profile: &mut Profile) -> TransitionState {
    clear_background(VIOLET);

    let label = format!("Gold: {}", profile.gold);
    root_ui().label(None, &label);
    let label = format!(
        "Equipped: {}, {}, {}",
        profile.equipped.pickaxe.name, profile.equipped.backpack.name, profile.equipped.hat.name
    );
    root_ui().label(None, &label);

    root_ui().label(None, "Items:");
    for item in &profile.items {
        let label = format!("{item}");
        root_ui().label(None, &label);
    }

    if root_ui().button(None, "Level 1") {
        return TransitionState::StartLevel(1);
    }
    if root_ui().button(None, "Level 2") {
        return TransitionState::StartLevel(2);
    }
    TransitionState::None
}
