#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowToken {
    NewLine,
    Comma,
    Dot,
    Colons,
}

impl ControlFlowToken {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "\n" => Some(ControlFlowToken::NewLine),
            "," => Some(ControlFlowToken::Comma),
            "." => Some(ControlFlowToken::Dot),
            ":" => Some(ControlFlowToken::Colons),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerAbilityMarker {
    Whenever,
    When,
    At,
}

impl TriggerAbilityMarker {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "whenever" => Some(TriggerAbilityMarker::Whenever),
            "when" => Some(TriggerAbilityMarker::When),
            "at" => Some(TriggerAbilityMarker::At),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapUntapCost {
    Tap,
    Untap,
}

impl TapUntapCost {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "{t}" => Some(TapUntapCost::Tap),
            "{q}" => Some(TapUntapCost::Untap),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnglishKeywords {
    Already,
    Additional,
    Among,
    And,
    AndOr,
    Another,
    Are,
    As,
    Be,
    Beginning,
    By,
    Cant,
    Copy,
    Divided,
    During,
    Do,
    Equal,
    Except,
    For,
    From,
    Has,
    Have,
    Into,
    If,
    In,
    Instead,
    Is,
    It,
    Kind,
    Less,
    May,
    More,
    No,
    Of,
    On,
    Only,
    Onto,
    Or,
    Other,
    RatherThan,
    Than,
    That,
    The,
    Them,
    Then,
    There,
    To,
    Top,
    Until,
    Was,
    Where,
    With,
    Without,
    Would,
}

impl EnglishKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "already" => Some(EnglishKeywords::Already),
            "additional" => Some(EnglishKeywords::Additional),
            "among" => Some(EnglishKeywords::Among),
            "and" => Some(EnglishKeywords::And),
            "and/or" => Some(EnglishKeywords::AndOr),
            "another" => Some(EnglishKeywords::Another),
            "as" => Some(EnglishKeywords::As),
            "are" => Some(EnglishKeywords::Are),
            "be" => Some(EnglishKeywords::Be),
            "beginning" => Some(EnglishKeywords::Beginning),
            "by" => Some(EnglishKeywords::By),
            "can't" => Some(EnglishKeywords::Cant),
            "copy" => Some(EnglishKeywords::Copy),
            "divided" => Some(EnglishKeywords::Divided),
            "during" => Some(EnglishKeywords::During),
            "do" => Some(EnglishKeywords::Do),
            "equal" => Some(EnglishKeywords::Equal),
            "except" => Some(EnglishKeywords::Except),
            "from" => Some(EnglishKeywords::From),
            "for" => Some(EnglishKeywords::For),
            "has" => Some(EnglishKeywords::Has),
            "have" => Some(EnglishKeywords::Have),
            "into" => Some(EnglishKeywords::Into),
            "if" => Some(EnglishKeywords::If),
            "in" => Some(EnglishKeywords::In),
            "instead" => Some(EnglishKeywords::Instead),
            "is" | "'s" => Some(EnglishKeywords::Is),
            "it" => Some(EnglishKeywords::It),
            "kind" => Some(EnglishKeywords::Kind),
            "less" => Some(EnglishKeywords::Less),
            "may" => Some(EnglishKeywords::May),
            "more" => Some(EnglishKeywords::More),
            "no" => Some(EnglishKeywords::No),
            "of" => Some(EnglishKeywords::Of),
            "on" => Some(EnglishKeywords::On),
            "only" => Some(EnglishKeywords::Only),
            "onto" => Some(EnglishKeywords::Onto),
            "or" => Some(EnglishKeywords::Or),
            "other" => Some(EnglishKeywords::Other),
            "rather than" => Some(EnglishKeywords::RatherThan),
            "than" => Some(EnglishKeywords::Than),
            "that" => Some(EnglishKeywords::That),
            "the" => Some(EnglishKeywords::The),
            "them" => Some(EnglishKeywords::Them),
            "then" => Some(EnglishKeywords::Then),
            "there" => Some(EnglishKeywords::There),
            "to" => Some(EnglishKeywords::To),
            "top" => Some(EnglishKeywords::Top),
            "until" => Some(EnglishKeywords::Until),
            "was" => Some(EnglishKeywords::Was),
            "where" => Some(EnglishKeywords::Where),
            "with" => Some(EnglishKeywords::With),
            "without" => Some(EnglishKeywords::Without),
            "would" => Some(EnglishKeywords::Would),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfReferencing;

impl SelfReferencing {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "this" => Some(SelfReferencing),
            "~" => Some(SelfReferencing),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberReference;

impl NumberReference {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "that many" | "number of" | "amount of" => Some(NumberReference),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotOfAKind;

impl NotOfAKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "non-" => Some(NotOfAKind),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKeywords {
    Deals,
    Gain,
    Get,
    Put,
    Reveal,
}

impl ActionKeywords {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "deal" | "deals" => Some(ActionKeywords::Deals),
            "gain" | "gains" => Some(ActionKeywords::Gain),
            "get" | "gets" => Some(ActionKeywords::Gain),
            "put" | "puts" => Some(ActionKeywords::Gain),
            "reveal" | "reveals" => Some(ActionKeywords::Gain),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageKind {
    Damage,
    CombatDamage,
    NoncombatDamage,
}

impl DamageKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "damage" | "damages" => Some(DamageKind::Damage),
            "combat damage" => Some(DamageKind::CombatDamage),
            "noncombat damage" => Some(DamageKind::NoncombatDamage),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerActions {
    Add,
    Attach,
    Attack,
    Cast,
    Change,
    Choose,
    Create,
    Destroy,
    Discard,
    Double,
    Draw,
    Exile,
    LookAt,
    Lose,
    Pay,
    Play,
    Put,
    Return,
    Sacrifice,
    Scry,
    Search,
    Shuffle,
    Spend,
    Tap,
    Untap,
}

impl PlayerActions {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "add" | "adds" => Some(PlayerActions::Add),
            "attach" | "attaches" => Some(PlayerActions::Attach),
            "attack" | "attacks" => Some(PlayerActions::Attack),
            "cast" | "casts" => Some(PlayerActions::Cast),
            "change" | "changes" => Some(PlayerActions::Change),
            "choose" | "chooses" | "choice" => Some(PlayerActions::Choose),
            "create" | "creates" => Some(PlayerActions::Create),
            "destroy" | "destroys" => Some(PlayerActions::Destroy),
            "discard" | "discards" => Some(PlayerActions::Discard),
            "double" | "doubles" => Some(PlayerActions::Double),
            "draw" | "draws" => Some(PlayerActions::Draw),
            "exile" | "exiles" => Some(PlayerActions::Exile),
            "look at" | "looks at" => Some(PlayerActions::LookAt),
            "lose" | "loses" => Some(PlayerActions::LookAt),
            "pay" | "pays" => Some(PlayerActions::Pay),
            "play" | "plays" => Some(PlayerActions::Play),
            "put" | "puts" => Some(PlayerActions::Play),
            "return" | "returns" => Some(PlayerActions::Return),
            "sacrifice" | "sacrifices" => Some(PlayerActions::Sacrifice),
            "scry" | "scrys" => Some(PlayerActions::Scry),
            "search" | "searchs" => Some(PlayerActions::Search),
            "shuffle" | "shuffles" => Some(PlayerActions::Shuffle),
            "spend" | "spends" => Some(PlayerActions::Spend),
            "tap" | "taps" => Some(PlayerActions::Tap),
            "untap" | "untaps" => Some(PlayerActions::Untap),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VhyToSortLater {
    Life,
    HandSize,
    MaximumHandSize,
    Source,
    Cost,
    Player,
    Turn,
    Mana,
    OpeningHand,
    Ability,
}

impl VhyToSortLater {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "ability" => Some(VhyToSortLater::Ability),
            "life" => Some(VhyToSortLater::Life),
            "mana" => Some(VhyToSortLater::Life),
            "player" => Some(VhyToSortLater::Player),
            "hand size" => Some(VhyToSortLater::HandSize),
            "maximum hand size" => Some(VhyToSortLater::HandSize),
            "opening hand" => Some(VhyToSortLater::OpeningHand),
            "source" => Some(VhyToSortLater::Source),
            "cost" | "costs" => Some(VhyToSortLater::Cost),
            "turn" | "turns" => Some(VhyToSortLater::Turn),
            _ => None,
        }
    }
}
