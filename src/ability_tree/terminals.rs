use std::str::FromStr;

pub trait Terminal: std::fmt::Display + Sized {
    fn try_from_str(source: &str) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Number(i32),
    X,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::X => write!(f, "x"),
            Number::Number(num) => write!(f, "{num}"),
        }
    }
}

impl Terminal for Number {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "x" => Some(Number::X),
            "a" => Some(Number::Number(1)),
            "one" => Some(Number::Number(1)),
            "two" => Some(Number::Number(2)),
            "three" => Some(Number::Number(3)),
            "four" => Some(Number::Number(4)),
            "five" => Some(Number::Number(5)),
            "six" => Some(Number::Number(6)),
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
    All,
    Target,
    UpTo(usize),
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountSpecifier::All => write!(f, "all"),
            CountSpecifier::Target => write!(f, "target"),
            CountSpecifier::UpTo(amount) => write!(f, "up to {amount}"),
        }
    }
}

impl Terminal for CountSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "all" => Some(CountSpecifier::All),
            "each" => Some(CountSpecifier::All),
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
    Attacks,
    Enters,
    Dies,
}

impl std::fmt::Display for CardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardActions::Attacks => write!(f, "attacks"),
            CardActions::Dies => write!(f, "dies"),
            CardActions::Enters => write!(f, "enters"),
        }
    }
}

impl Terminal for CardActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attacks" => Some(CardActions::Attacks),
            "dies" => Some(CardActions::Dies),
            "enters" => Some(CardActions::Enters),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerActions {
    Add,
    Attack,
    Cast,
    Choose,
    Create,
    Destroy,
    Draw,
    Exile,
    Pay,
    Play,
    Scry,
    Search,
}

impl std::fmt::Display for PlayerActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerActions::Add => write!(f, "add"),
            PlayerActions::Attack => write!(f, "attack"),
            PlayerActions::Cast => write!(f, "cast"),
            PlayerActions::Choose => write!(f, "choose"),
            PlayerActions::Create => write!(f, "create"),
            PlayerActions::Destroy => write!(f, "destroy"),
            PlayerActions::Draw => write!(f, "draw"),
            PlayerActions::Exile => write!(f, "exile"),
            PlayerActions::Pay => write!(f, "pay"),
            PlayerActions::Play => write!(f, "play"),
            PlayerActions::Scry => write!(f, "scry"),
            PlayerActions::Search => write!(f, "search"),
        }
    }
}

impl Terminal for PlayerActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "add" => Some(PlayerActions::Add),
            "attack" => Some(PlayerActions::Attack),
            "cast" => Some(PlayerActions::Cast),
            "choose" => Some(PlayerActions::Choose),
            "create" => Some(PlayerActions::Create),
            "destroy" => Some(PlayerActions::Destroy),
            "draw" => Some(PlayerActions::Draw),
            "exile" => Some(PlayerActions::Exile),
            "pay" => Some(PlayerActions::Pay),
            "play" => Some(PlayerActions::Play),
            "scry" => Some(PlayerActions::Scry),
            "search" => Some(PlayerActions::Search),
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

impl Terminal for mtg_data::Mana {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::Mana::from_str(source).ok()
    }
}
