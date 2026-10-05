use crate::{
    constants::SAVE_SLOT_COUNT,
    game::{GameState, TransitionState},
};
use macroquad::{
    color::*,
    ui::{hash, root_ui},
    window::clear_background,
};

pub fn main_menu_update() -> TransitionState {
    clear_background(MAROON);
    if root_ui().button(None, "Play") {
        return TransitionState::GoTo(GameState::LoadSave(LoadSaveState::new()));
    }
    if root_ui().button(None, "Settings") {
        return TransitionState::GoTo(GameState::Settings);
    }
    TransitionState::None
}

pub fn settings_update() -> TransitionState {
    clear_background(BROWN);
    if root_ui().button(None, "< Main Menu") {
        return TransitionState::GoTo(GameState::MainMenu);
    }
    TransitionState::None
}

pub struct SlotInput {
    pub name: String,
}
pub struct LoadSaveState {
    pub slots: Vec<SlotInput>,
}

impl LoadSaveState {
    pub fn new() -> Self {
        Self {
            slots: (0..SAVE_SLOT_COUNT)
                .map(|_| SlotInput {
                    name: String::new(),
                })
                .collect(),
        }
    }
}

pub fn load_save_update(state: &mut LoadSaveState) -> TransitionState {
    clear_background(BEIGE);

    for (i, slot) in state.slots.iter_mut().enumerate() {
        root_ui().label(None, &format!("Slot {}", i + 1));
        root_ui().input_text(hash!("slot_name", i + 1), "Name", &mut slot.name);
        if root_ui().button(None, "Save") && !slot.name.trim().is_empty() {
            return TransitionState::LoadProfile {
                slot: i + 1,
                name: slot.name.clone(),
            };
        }
    }

    if root_ui().button(None, "< Main Menu") {
        return TransitionState::GoTo(GameState::MainMenu);
    }

    TransitionState::None
}
