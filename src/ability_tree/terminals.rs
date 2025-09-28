pub trait Terminal: std::fmt::Display + Sized {
    fn try_from_str(source: &str) -> Option<(Self, usize)>;
}

pub trait KeywordTerminal {
    fn repr(&self) -> &'static str;
    fn iter() -> impl Iterator<Item = Self>;
}

impl<T: KeywordTerminal + std::fmt::Display> Terminal for T {
    fn try_from_str(source: &str) -> Option<(Self, usize)> {
        for item in Self::iter() {
            if source.starts_with(item.repr()) {
                let length = item.repr().len();
                return Some((item, length));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    A,
    Number(i32),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::A => write!(f, "A"),
            Number::Number(num) => write!(f, "{num}"),
        }
    }
}

impl Terminal for Number {
    fn try_from_str(source: &str) -> Option<(Self, usize)> {
        const DECIMALS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        if source.starts_with("a") {
            Some((Number::A, "a".len()))
        } else if source.starts_with(&DECIMALS) {
            let mut decimal_count = 1;
            while decimal_count < source.len() && source[decimal_count..].starts_with(&DECIMALS) {
                decimal_count += 1;
            }
            let num: i32 = source[0..decimal_count].parse().ok()?;
            Some((Number::Number(num), decimal_count))
        } else {
            None
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
            Counter::PlusOne => write!(f, "+1/+1 Counter"),
        }
    }
}

impl KeywordTerminal for Counter {
    fn repr(&self) -> &'static str {
        match self {
            Counter::PlusOne => "+1/+1",
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [Counter::PlusOne].into_iter()
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
            CountSpecifier::Each => write!(f, "Each"),
            CountSpecifier::Target => write!(f, "Target"),
            CountSpecifier::UpTo(amount) => write!(f, "Up To {amount}"),
        }
    }
}

impl Terminal for CountSpecifier {
    fn try_from_str(source: &str) -> Option<(Self, usize)> {
        if source.starts_with("each") {
            Some((CountSpecifier::Each, "each".len()))
        } else if source.starts_with("target") {
            Some((CountSpecifier::Target, "target".len()))
        } else if source.starts_with("up to ") {
            const DECIMALS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let source = &source["up to ".len()..];
            if source.starts_with(&DECIMALS) {
                let mut decimal_count = 1;
                while decimal_count < source.len() && source[decimal_count..].starts_with(&DECIMALS)
                {
                    decimal_count += 1;
                }
                let num: usize = source[0..decimal_count].parse().ok()?;
                Some((CountSpecifier::UpTo(num), "up to ".len() + decimal_count))
            } else {
                None
            }
        } else {
            None
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
            ControlSpecifier::YouControl => write!(f, "You Control"),
            ControlSpecifier::YouDontControl => write!(f, "You Don't Control"),
        }
    }
}

impl KeywordTerminal for ControlSpecifier {
    fn repr(&self) -> &'static str {
        match self {
            ControlSpecifier::YouControl => "you control",
            ControlSpecifier::YouDontControl => "you don't control",
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [
            ControlSpecifier::YouControl,
            ControlSpecifier::YouDontControl,
        ]
        .into_iter()
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
            Appartenance::Your => write!(f, "Your"),
            Appartenance::AnOpponent => write!(f, "An Opponent's"),
        }
    }
}

impl KeywordTerminal for Appartenance {
    fn repr(&self) -> &'static str {
        match self {
            Appartenance::Your => "your",
            Appartenance::AnOpponent => "an opponent",
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [Appartenance::Your, Appartenance::AnOpponent].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Actions {
    Dies,
    Attacks,
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::Dies => write!(f, "Dies"),
            Actions::Attacks => write!(f, "Attacks"),
        }
    }
}

impl KeywordTerminal for Actions {
    fn repr(&self) -> &'static str {
        match self {
            Actions::Dies => "dies",
            Actions::Attacks => "attacks",
        }
    }
    fn iter() -> impl Iterator<Item = Self> {
        [Actions::Dies, Actions::Attacks].into_iter()
    }
}
