use crate::{
    game::{GameState, TransitionState},
    level::levels::LEVELS,
    save::Profile,
};
use macroquad::{color::*, ui::root_ui, window::clear_background};

pub fn hub_update(profile: &mut Profile) -> TransitionState {
    clear_background(VIOLET);

    if root_ui().button(None, "< Main Menu") {
        return TransitionState::GoTo(GameState::MainMenu);
    }

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

    for level in LEVELS {
        let label = format!("Level {}", level.id + 1);
        if root_ui().button(None, label.as_str()) {
            return TransitionState::StartLevel(level.id);
        }
    }

    TransitionState::None
}
