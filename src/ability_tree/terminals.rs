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
            "an" => Some(Number::Number(1)),
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
            "+1/+1 counter" | "+1/+1 counters" => Some(Counter::PlusOne),
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
    Discard,
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
            PlayerActions::Discard => write!(f, "discard"),
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
            "add" | "adds" => Some(PlayerActions::Add),
            "attack" | "attacks" => Some(PlayerActions::Attack),
            "cast" | "casts" => Some(PlayerActions::Cast),
            "choose" | "chooses" => Some(PlayerActions::Choose),
            "create" | "creates" => Some(PlayerActions::Create),
            "destroy" | "destroys" => Some(PlayerActions::Destroy),
            "discard" | "discards" => Some(PlayerActions::Discard),
            "draw" | "draws" => Some(PlayerActions::Draw),
            "exile" | "exiles" => Some(PlayerActions::Exile),
            "pay" | "pays" => Some(PlayerActions::Pay),
            "play" | "plays" => Some(PlayerActions::Play),
            "scry" | "scrys" => Some(PlayerActions::Scry),
            "search" | "searchs" => Some(PlayerActions::Search),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermanentProperty {
    Blocking,
    Attacking,
    Tapped,
    Untapped,
}

impl std::fmt::Display for PermanentProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentProperty::Blocking => write!(f, "blocking"),
            PermanentProperty::Attacking => write!(f, "attacking"),
            PermanentProperty::Tapped => write!(f, "tapped"),
            PermanentProperty::Untapped => write!(f, "untapped"),
        }
    }
}

impl Terminal for PermanentProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "blocking" => Some(PermanentProperty::Blocking),
            "attacking" => Some(PermanentProperty::Attacking),
            "tapped" => Some(PermanentProperty::Tapped),
            "untapped" => Some(PermanentProperty::Untapped),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellProperty {
    Countered,
}

impl std::fmt::Display for SpellProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellProperty::Countered => write!(f, "countered"),
        }
    }
}

impl Terminal for SpellProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "countered" => Some(SpellProperty::Countered),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PowerToughness {
    power: u32,
    toughness: u32,
}

impl std::fmt::Display for PowerToughness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.power, self.toughness)
    }
}

impl Terminal for PowerToughness {
    fn try_from_str(source: &str) -> Option<Self> {
        let split: Vec<_> = source.split('/').collect();
        let (raw_pow, raw_tough) = match split.as_slice() {
            [pow, tough] => (pow, tough),
            _ => return None,
        };
        if !crate::utils::is_digits(raw_pow) {
            return None;
        }
        if !crate::utils::is_digits(raw_tough) {
            return None;
        }
        Some(PowerToughness {
            power: raw_pow.parse().ok()?,
            toughness: raw_tough.parse().ok()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PowerToughnessModifier {
    power: i32,
    toughness: i32,
}

impl std::fmt::Display for PowerToughnessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}/{:+}", self.power, self.toughness)
    }
}

impl Terminal for PowerToughnessModifier {
    fn try_from_str(source: &str) -> Option<Self> {
        let split: Vec<_> = source.split('/').collect();
        let (raw_pow, raw_tough) = match split.as_slice() {
            [pow, tough] => (pow, tough),
            _ => return None,
        };
        if !raw_pow.starts_with(&['+', '-']) {
            return None;
        }
        if !crate::utils::is_digits(&raw_pow[1..]) {
            return None;
        }
        if !raw_tough.starts_with(&['+', '-']) {
            return None;
        }
        if !crate::utils::is_digits(&raw_tough[1..]) {
            return None;
        }
        Some(PowerToughnessModifier {
            power: raw_pow.parse().ok()?,
            toughness: raw_tough.parse().ok()?,
        })
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

impl Terminal for mtg_data::CardType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::CardType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::CreatureType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::CreatureType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::EnchantmentType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::EnchantmentType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::LandType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::LandType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::PlaneswalkerType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::PlaneswalkerType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::BattleType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::BattleType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::ArtifactType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::ArtifactType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::SpellType {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::SpellType::from_str(source).ok()
    }
}

impl Terminal for mtg_data::Supertype {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::Supertype::from_str(source).ok()
    }
}

impl Terminal for mtg_data::Color {
    fn try_from_str(source: &str) -> Option<Self> {
        mtg_data::Color::from_str(source).ok()
    }
}
