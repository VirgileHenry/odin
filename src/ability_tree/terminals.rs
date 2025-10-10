mod counter;
mod mtg_data;

pub use counter::Counter;

pub trait Terminal: std::fmt::Display + Sized {
    fn try_from_str(source: &str) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Number(u32),
    X,
    OrMore(u32),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::X => write!(f, "x"),
            Number::Number(num) => write!(f, "{num}"),
            Number::OrMore(num) => write!(f, "{num} or more"),
        }
    }
}

impl Terminal for Number {
    fn try_from_str(source: &str) -> Option<Self> {
        if let Some(num) = crate::utils::parse_num(source) {
            Some(Number::Number(num))
        } else if source == "x" {
            Some(Number::X)
        } else if let Some(stripped) = source.strip_suffix(" or more") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Number::OrMore(num))
        } else if let Some(stripped) = source.strip_suffix(" or greater") {
            let num = crate::utils::parse_num(stripped)?;
            Some(Number::OrMore(num))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CountSpecifier {
    All,
    Target,
    UpTo(u32),
    AnyNumberOfTargets,
}

impl std::fmt::Display for CountSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountSpecifier::All => write!(f, "all"),
            CountSpecifier::Target => write!(f, "target"),
            CountSpecifier::UpTo(amount) => write!(f, "up to {amount}"),
            CountSpecifier::AnyNumberOfTargets => write!(f, "any number of target"),
        }
    }
}

impl Terminal for CountSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "all" => Some(CountSpecifier::All),
            "each" => Some(CountSpecifier::All),
            "target" => Some(CountSpecifier::Target),
            "any target" => Some(CountSpecifier::Target),
            "any number of target" => Some(CountSpecifier::AnyNumberOfTargets),
            other => {
                let prefix = "up to ";
                if other.starts_with(prefix) {
                    let num = crate::utils::parse_num(&other[prefix.len()..])?;
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
            "you control" | "you already control" => Some(ControlSpecifier::YouControl),
            "you don't control" | "your opponents control" | "an opponent controls" => {
                Some(ControlSpecifier::YouDontControl)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnerSpecifier {
    YouOwn,
    YouDontOwn,
    ObjectOwner,
}

impl std::fmt::Display for OwnerSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerSpecifier::YouOwn => write!(f, "you own"),
            OwnerSpecifier::YouDontOwn => write!(f, "you don't own"),
            OwnerSpecifier::ObjectOwner => write!(f, "it's owner"),
        }
    }
}

impl Terminal for OwnerSpecifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "you own" => Some(OwnerSpecifier::YouOwn),
            "you don't own" => Some(OwnerSpecifier::YouDontOwn),
            "it's owner" => Some(OwnerSpecifier::ObjectOwner),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Order {
    RandomOrder,
    ChosenOrder,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::RandomOrder => write!(f, "a random order"),
            Order::ChosenOrder => write!(f, "any order"),
        }
    }
}

impl Terminal for Order {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "a random order" => Some(Order::RandomOrder),
            "any order" => Some(Order::ChosenOrder),
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
            "their" => Some(Appartenance::AnOpponent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardActions {
    Attacks,
    Blocks,
    Dies,
    Enters,
    Fight,
}

impl std::fmt::Display for CardActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardActions::Attacks => write!(f, "attacks"),
            CardActions::Blocks => write!(f, "blocks"),
            CardActions::Dies => write!(f, "dies"),
            CardActions::Enters => write!(f, "enters"),
            CardActions::Fight => write!(f, "fights"),
        }
    }
}

impl Terminal for CardActions {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attack" | "attacks" => Some(CardActions::Attacks),
            "block" | "blocks" => Some(CardActions::Attacks),
            "die" | "dies" => Some(CardActions::Dies),
            "enter" | "enters" => Some(CardActions::Enters),
            "fight" | "fights" => Some(CardActions::Fight),
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
            "an opponent" | "each opponent" => Some(PlayerSpecifier::AnOpponent),
            "a player" | "each player" => Some(PlayerSpecifier::Any),
            "the player to your left" => Some(PlayerSpecifier::ToYourLeft),
            "the player to your right" => Some(PlayerSpecifier::ToYourRight),
            "you" => Some(PlayerSpecifier::You),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermanentProperty {
    Power,
    Tougness,
    ConvertedManaCost,
}

impl std::fmt::Display for PermanentProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentProperty::Power => write!(f, "power"),
            PermanentProperty::Tougness => write!(f, "touhness"),
            PermanentProperty::ConvertedManaCost => write!(f, "converted mana cost"),
        }
    }
}

impl Terminal for PermanentProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "power" => Some(PermanentProperty::Power),
            "toughness" => Some(PermanentProperty::Tougness),
            "mana cost" | "mana value" => Some(PermanentProperty::ConvertedManaCost),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PermanentState {
    Attacking,
    Blocking,
    Blocked,
    Equipped,
    Tapped,
    Untapped,
}

impl std::fmt::Display for PermanentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermanentState::Attacking => write!(f, "attacking"),
            PermanentState::Blocking => write!(f, "blocking"),
            PermanentState::Blocked => write!(f, "blocked"),
            PermanentState::Tapped => write!(f, "tapped"),
            PermanentState::Untapped => write!(f, "untapped"),
            PermanentState::Equipped => write!(f, "equipped"),
        }
    }
}

impl Terminal for PermanentState {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "attacking" => Some(PermanentState::Attacking),
            "blocking" => Some(PermanentState::Blocking),
            "blocked" => Some(PermanentState::Blocked),
            "tapped" => Some(PermanentState::Tapped),
            "untapped" => Some(PermanentState::Untapped),
            "equipped" => Some(PermanentState::Equipped),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpellProperty {
    Countered,
    Kicked,
}

impl std::fmt::Display for SpellProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellProperty::Countered => write!(f, "countered"),
            SpellProperty::Kicked => write!(f, "kicked"),
        }
    }
}

impl Terminal for SpellProperty {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "countered" => Some(SpellProperty::Countered),
            "kicked" => Some(SpellProperty::Countered),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Phase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    End,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Beginning => write!(f, "beginning phase"),
            Phase::PrecombatMain => write!(f, "precombat main phase"),
            Phase::Combat => write!(f, "combat phase"),
            Phase::PostcombatMain => write!(f, "postcombat main phase"),
            Phase::End => write!(f, "end phase"),
        }
    }
}

impl Terminal for Phase {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "beginning phase" => Some(Phase::Beginning),
            "precombat main phase" => Some(Phase::PrecombatMain),
            "combat phase" => Some(Phase::Combat),
            "postcombat main phase" => Some(Phase::PostcombatMain),
            "end phase" => Some(Phase::End),
            "end of turn" => Some(Phase::End),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Step {
    Untap,
    Upkeep,
    Draw,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeDamage,
    Damage,
    LastStrikeDamage,
    EndOfCombat,
    End,
    Cleanup,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Untap => write!(f, "untap"),
            Step::Upkeep => write!(f, "upkeep"),
            Step::Draw => write!(f, "draw"),
            Step::BeginningOfCombat => write!(f, "beginning of combat"),
            Step::DeclareAttackers => write!(f, "declaration of attackers"),
            Step::DeclareBlockers => write!(f, "declaration of blockers"),
            Step::FirstStrikeDamage => write!(f, "first strike damage step"),
            Step::Damage => write!(f, "damage step"),
            Step::LastStrikeDamage => write!(f, "last strike damage step"),
            Step::EndOfCombat => write!(f, "end of combat"),
            Step::End => write!(f, "end step"),
            Step::Cleanup => write!(f, "cleanup"),
        }
    }
}

impl Terminal for Step {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "untap step" => Some(Step::Untap),
            "upkeep" => Some(Step::Upkeep),
            "draw step" => Some(Step::Draw),
            "beginning of combat" => Some(Step::BeginningOfCombat),
            "declaration of attackers" => Some(Step::DeclareAttackers),
            "declaration of blockers" => Some(Step::DeclareBlockers),
            "first strike damage step" => Some(Step::FirstStrikeDamage),
            "damage step" => Some(Step::Damage),
            "last strike damage step" => Some(Step::LastStrikeDamage),
            "end of combat" => Some(Step::EndOfCombat),
            "end step" => Some(Step::End),
            "cleanup" => Some(Step::Cleanup),
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
pub enum PowerToughnessModifier {
    Constant { power: i32, toughness: i32 },
    PlusXPlusX,
    PlusXMinusX,
    MinusXPlusX,
}

impl std::fmt::Display for PowerToughnessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerToughnessModifier::Constant { power, toughness } => {
                write!(f, "{:+}/{:+}", power, toughness)
            }
            PowerToughnessModifier::PlusXPlusX => write!(f, "+x/+x"),
            PowerToughnessModifier::PlusXMinusX => write!(f, "+x/-x"),
            PowerToughnessModifier::MinusXPlusX => write!(f, "-x/+x"),
        }
    }
}

impl Terminal for PowerToughnessModifier {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "+x/+x" => Some(PowerToughnessModifier::PlusXPlusX),
            "+x/-x" => Some(PowerToughnessModifier::PlusXMinusX),
            "-x/+x" => Some(PowerToughnessModifier::MinusXPlusX),
            other => {
                let split: Vec<_> = other.split('/').collect();
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
                Some(PowerToughnessModifier::Constant {
                    power: raw_pow.parse().ok()?,
                    toughness: raw_tough.parse().ok()?,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlaneswalkerAbilityCost(i32);

impl std::fmt::Display for PlaneswalkerAbilityCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

impl Terminal for PlaneswalkerAbilityCost {
    fn try_from_str(source: &str) -> Option<Self> {
        if !source.starts_with(&['+', '-']) {
            return None;
        }
        if !crate::utils::is_digits(&source[1..]) {
            return None;
        }
        Some(PlaneswalkerAbilityCost(source.parse().ok()?))
    }
}
