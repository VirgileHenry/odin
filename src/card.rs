mod game;
mod legalities;
mod printed;

pub use game::Card;
pub use printed::PrintedCard;

pub type CardName = arrayvec::ArrayString<128>;
pub type ColorIdentity = arrayvec::ArrayVec<mtg_data::Color, 5>;
pub type ManaCost = arrayvec::ArrayVec<mtg_data::Mana, 16>;
