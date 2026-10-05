use crate::level::winnings::Winnings;

pub enum Outcome {
    Win(Winnings),
    Lose,
    Exit,
}
