use std::str::FromStr;

pub trait Terminal: std::fmt::Display + Sized {
    fn try_from_str(source: &str) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    A,
    Number(i32),
    X,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::A => write!(f, "a"),
            Number::X => write!(f, "x"),
            Number::Number(num) => write!(f, "{num}"),
        }
    }
}

impl Terminal for Number {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "a" => Some(Number::A),
            "x" => Some(Number::X),
            other => {
                let num = other.parse().ok()?;
                Some(Number::Number(num))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Counter {
    PlusOne,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Counter::PlusOne => write!(f, "+1/+1 counter"),
        }
    }
}

impl Terminal for Counter {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "+1/+1 counter" => Some(Counter::PlusOne),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CountSpecifier {
    Each,
    Target,
    UpTo(usize),
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountSpecifier::Each => write!(f, "each"),
            CountSpecifier::Target => write!(f, "target"),
            CountSpecifier::UpTo(amount) => write!(f, "up to {amount}"),
        }
    }
}

impl Terminal for CountSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "each" => Some(CountSpecifier::Each),
            "target" => Some(CountSpecifier::Target),
            other => {
                let prefix = "up to ";
                if other.starts_with(prefix) {
                    let decimal_part = &other[prefix.len()..];
                    let num = decimal_part.parse().ok()?;
                    Some(CountSpecifier::UpTo(num))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlSpecifier {
    YouControl,
    YouDontControl,
}

impl std::fmt::Display for ControlSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlSpecifier::YouControl => write!(f, "you control"),
            ControlSpecifier::YouDontControl => write!(f, "you don't control"),
        }
    }
}

impl Terminal for ControlSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you control" => Some(ControlSpecifier::YouControl),
            "you don't control" => Some(ControlSpecifier::YouDontControl),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Appartenance {
    Your,
    AnOpponent,
}

impl std::fmt::Display for Appartenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appartenance::Your => write!(f, "your"),
            Appartenance::AnOpponent => write!(f, "an opponent's"),
        }
    }
}

impl Terminal for Appartenance {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "your" => Some(Appartenance::Your),
            "an opponent" => Some(Appartenance::AnOpponent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    Dies,
    Attacks,
}

impl std::fmt::Display for CardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardActions::Dies => write!(f, "dies"),
            CardActions::Attacks => write!(f, "attacks"),
        }
    }
}

impl Terminal for CardActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "dies" => Some(CardActions::Dies),
            "attacks" => Some(CardActions::Attacks),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerActions {
    Exile,
    Cast,
    Play,
    Attacks,
    Scry,
    Draw,
}

impl std::fmt::Display for PlayerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerActions::Attacks => write!(f, "attacks"),
            PlayerActions::Cast => write!(f, "cast"),
            PlayerActions::Draw => write!(f, "draw"),
            PlayerActions::Exile => write!(f, "exile"),
            PlayerActions::Play => write!(f, "play"),
            PlayerActions::Scry => write!(f, "scry"),
        }
    }
}

impl Terminal for PlayerActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attacks" => Some(PlayerActions::Attacks),
            "cast" => Some(PlayerActions::Cast),
            "draw" => Some(PlayerActions::Draw),
            "exile" => Some(PlayerActions::Exile),
            "play" => Some(PlayerActions::Play),
            "scry" => Some(PlayerActions::Scry),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerSpecifier {
    AnOpponent,
    Any,
    ToYourLeft,
    ToYourRight,
    You,
}

impl std::fmt::Display for PlayerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerSpecifier::AnOpponent => write!(f, "an opponent"),
            PlayerSpecifier::Any => write!(f, "a player"),
            PlayerSpecifier::ToYourLeft => write!(f, "the player to your left"),
            PlayerSpecifier::ToYourRight => write!(f, "the player to your right"),
            PlayerSpecifier::You => write!(f, "you"),
        }
    }
}

impl Terminal for PlayerSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "an opponent" => Some(PlayerSpecifier::AnOpponent),
            "a player" => Some(PlayerSpecifier::Any),
            "the player to your left" => Some(PlayerSpecifier::ToYourLeft),
            "the player to your right" => Some(PlayerSpecifier::ToYourRight),
            "you" => Some(PlayerSpecifier::You),
            _ => None,
        }
    }
}

impl Terminal for mtg_data::KeywordAbility {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::KeywordAbility::from_str(source).ok()
    }
}
