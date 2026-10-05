use crate::{
    level_session::{LevelSession, outcome::Outcome},
    levels::LevelId,
    main_menu::LoadSaveState,
    save::{Equipment, Profile, STARTING_BACKPACK, STARTING_HAT, STARTING_PICK},
};

pub enum GameState {
    MainMenu,
    Settings,
    LoadSave(LoadSaveState),
    Hub,
    Level(Box<LevelSession>),
}

pub enum TransitionState {
    None,
    GoTo(GameState),
    LoadProfile { slot: usize, name: String },
    StartLevel(LevelId),
    EndLevel(Outcome),
}

pub struct Game {
    profile: Option<Profile>,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            profile: None,
            state: GameState::MainMenu,
        }
    }

    pub fn get_state(&mut self) -> &mut GameState {
        &mut self.state
    }

    pub fn set_state(&mut self, to_state: TransitionState) {
        match to_state {
            TransitionState::None => {}
            TransitionState::GoTo(to) => self.state = to,
            TransitionState::LoadProfile { slot: s, name: n } => self.load_profile(s, n),
            TransitionState::StartLevel(id) => self.start_level(id),
            TransitionState::EndLevel(result) => self.end_level(result),
        }
    }

    pub fn get_profile(&mut self) -> &mut Profile {
        self.profile.as_mut().expect("Profile does not exist.")
    }

    fn load_profile(&mut self, s: usize, n: String) {
        if self.profile.is_none() {
            self.init_profile(n, s);
        }
        self.state = GameState::Hub;
    }

    fn init_profile(&mut self, profile_name: String, slot: usize) {
        self.profile = Some(Profile {
            name: profile_name,
            slot,
            gold: 0,
            items: vec![],
            equipped: Equipment {
                pickaxe: &STARTING_PICK,
                hat: &STARTING_HAT,
                backpack: &STARTING_BACKPACK,
            },
        })
    }

    fn start_level(&mut self, id: LevelId) {
        let p = self.get_profile();
        let starting_energy = p.starting_energy();
        let starting_tech = p.starting_tech();
        let mining_power = p.get_mining_power();
        let level = LevelSession::new(id, starting_energy, starting_tech, mining_power);
        self.state = GameState::Level(Box::new(level));
    }

    fn end_level(&mut self, result: Outcome) {
        match result {
            Outcome::Win(winnings) => self.get_profile().collect_winnings(winnings),
            Outcome::Lose => {}
            Outcome::Exit => {}
        }
        self.state = GameState::Hub;
    }
}
