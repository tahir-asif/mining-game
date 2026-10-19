use crate::{
    hub::Hub,
    level::{Level, levels::LevelId, outcome::Outcome},
    main_menu::LoadSaveState,
    profile::Profile,
};

pub enum GameState {
    MainMenu,
    Settings,
    LoadSave(LoadSaveState),
    Hub,
    Level(Box<Level>),
    Quit,
}

pub enum TransitionState {
    None,
    GoTo(GameState),
    LoadProfile { slot: u8, name: String },
    StartLevel(LevelId),
    EndLevel(Outcome),
    Quit,
}

pub struct Game {
    state: GameState,
    profile: Option<Profile>,
    hub: Option<Hub>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::MainMenu,
            profile: None,
            hub: None,
        }
    }

    // State
    pub fn get_state(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub fn set_state(&mut self, to_state: TransitionState) {
        match to_state {
            TransitionState::None => {}
            TransitionState::GoTo(to) => self.verify_goto(to),
            TransitionState::LoadProfile { slot, name } => self.load_profile(slot, name),
            TransitionState::StartLevel(id) => self.start_level(id),
            TransitionState::EndLevel(result) => self.end_level(result),
            TransitionState::Quit => {
                self.persist_profile();
                self.state = GameState::Quit;
            }
        }
    }

    fn verify_goto(&mut self, to: GameState) {
        match to {
            GameState::Settings => self.state = GameState::Settings,
            GameState::LoadSave(data) => self.state = GameState::LoadSave(data),
            GameState::MainMenu => {
                self.persist_profile();
                self.profile = None;
                self.hub = None;
                self.state = GameState::MainMenu
            }
            _ => {}
        }
    }

    // Profile
    pub fn get_profile(&mut self) -> &mut Profile {
        self.profile.as_mut().expect("Profile does not exist.")
    }

    fn load_profile(&mut self, slot: u8, name: String) {
        if self.profile.is_some() {
            return;
        }
        self.profile = Some(match crate::save::load_profile(slot) {
            Ok(Some(p)) => p,
            Ok(None) => Profile::new(name, slot),
            Err(e) => {
                macroquad::prelude::error!("failed to load slot {}: {}", slot, e);
                Profile::new(name, slot)
            }
        });
        self.state = GameState::Hub;
        self.hub = Some(Hub::new());
    }

    pub fn persist_profile(&self) {
        if let Some(profile) = &self.profile {
            if let Err(e) = crate::save::save_profile(profile) {
                macroquad::prelude::error!("failed to save profile: {}", e);
            }
        }
    }

    // Hub
    pub fn hub_update(&mut self) -> TransitionState {
        let hub = self.hub.as_mut().expect("Hub does not exist.");
        let profile = self.profile.as_mut().expect("Profile does not exist.");
        hub.hub_update(profile)
    }

    // Level transition helper functions
    fn start_level(&mut self, id: LevelId) {
        let p = self.get_profile();
        let starting_energy = p.starting_energy();
        let starting_tech = p.starting_tech();
        let mining_power = p.get_mining_power();
        let level = Level::new(id, starting_energy, starting_tech, mining_power);
        self.state = GameState::Level(Box::new(level));
    }

    fn end_level(&mut self, result: Outcome) {
        match result {
            Outcome::Win(winnings) => {
                let profile = self.get_profile();
                profile.collect_winnings(winnings);
                profile.latest_completed_level += 1;
            }
            Outcome::Lose => {}
            Outcome::Exit => {}
        }
        self.persist_profile();
        self.state = GameState::Hub;
    }
}
