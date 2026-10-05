use crate::level_session::winnings::Winnings;
pub enum Outcome {
    Win(Winnings),
    Lose,
    Exit,
}

