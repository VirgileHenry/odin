mod non_terminals;

use crate::ability_tree::terminals::Terminal;
use crate::ability_tree::{object, terminals, zone};
use crate::lexer::span::Span;

fn gen_parse_func<T, F>(func: F) -> impl for<'src> Fn(Span<'src>) -> Option<Token<'src>>
where
    T: Into<TokenKind>,
    F: Fn(&str) -> Option<T>,
{
    return move |span| {
        let token: T = func(span.text)?;
        let kind: TokenKind = token.into();
        Some(Token { kind, span })
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    kind: TokenKind,
    span: crate::lexer::span::Span<'src>,
}

impl<'src> Token<'src> {
    pub fn try_from_str(span: Span<'src>) -> Option<Token<'src>> {
        let parse_funcs: &[&dyn Fn(Span<'src>) -> Option<Token<'src>>] = &[
            &gen_parse_func(terminals::Number::try_from_str),
            &gen_parse_func(terminals::Counter::try_from_str),
            &gen_parse_func(terminals::CountSpecifier::try_from_str),
            &gen_parse_func(terminals::ControlSpecifier::try_from_str),
            &gen_parse_func(terminals::OwnerSpecifier::try_from_str),
            &gen_parse_func(terminals::Order::try_from_str),
            &gen_parse_func(terminals::Appartenance::try_from_str),
            &gen_parse_func(terminals::CardActions::try_from_str),
            &gen_parse_func(terminals::PlayerSpecifier::try_from_str),
            &gen_parse_func(terminals::PermanentState::try_from_str),
            &gen_parse_func(terminals::PermanentProperty::try_from_str),
            &gen_parse_func(terminals::SpellProperty::try_from_str),
            &gen_parse_func(terminals::Phase::try_from_str),
            &gen_parse_func(terminals::Step::try_from_str),
            &gen_parse_func(terminals::PowerToughness::try_from_str),
            &gen_parse_func(terminals::PowerToughnessModifier::try_from_str),
            &gen_parse_func(terminals::PlaneswalkerAbilityCost::try_from_str),
            &gen_parse_func(object::Object::try_from_str),
            &gen_parse_func(zone::Zone::try_from_str),
            &gen_parse_func(mtg_data::Color::try_from_str),
            &gen_parse_func(mtg_data::KeywordAbility::try_from_str),
            &gen_parse_func(mtg_data::Mana::try_from_str),
            &gen_parse_func(mtg_data::CardType::try_from_str),
            &gen_parse_func(mtg_data::CreatureType::try_from_str),
            &gen_parse_func(mtg_data::EnchantmentType::try_from_str),
            &gen_parse_func(mtg_data::LandType::try_from_str),
            &gen_parse_func(mtg_data::PlaneswalkerType::try_from_str),
            &gen_parse_func(mtg_data::BattleType::try_from_str),
            &gen_parse_func(mtg_data::ArtifactType::try_from_str),
            &gen_parse_func(mtg_data::SpellType::try_from_str),
            &gen_parse_func(mtg_data::Supertype::try_from_str),
            &gen_parse_func(non_terminals::ControlFlowToken::try_from_str),
            &gen_parse_func(non_terminals::TriggerAbilityMarker::try_from_str),
            &gen_parse_func(non_terminals::TapUntapCost::try_from_str),
            &gen_parse_func(non_terminals::EnglishKeywords::try_from_str),
            &gen_parse_func(non_terminals::SelfReferencing::try_from_str),
            &gen_parse_func(non_terminals::NumberReference::try_from_str),
            &gen_parse_func(non_terminals::NotOfAKind::try_from_str),
            &gen_parse_func(non_terminals::ActionKeywords::try_from_str),
            &gen_parse_func(non_terminals::DamageKind::try_from_str),
            &gen_parse_func(non_terminals::PlayerActions::try_from_str),
            &gen_parse_func(non_terminals::VhyToSortLater::try_from_str),
        ];

        for parse_func in parse_funcs.into_iter() {
            if let Some(res) = parse_func(span) {
                return Some(res);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Number(terminals::Number),
    Counter(terminals::Counter),
    CountSpecifier(terminals::CountSpecifier),
    ControlSpecifier(terminals::ControlSpecifier),
    OwnerSpecifier(terminals::OwnerSpecifier),
    Order(terminals::Order),
    Appartenance(terminals::Appartenance),
    CardActions(terminals::CardActions),
    PlayerSpecifier(terminals::PlayerSpecifier),
    PermanentState(terminals::PermanentState),
    PermanentProperty(terminals::PermanentProperty),
    Phase(terminals::Phase),
    Step(terminals::Step),
    SpellProperty(terminals::SpellProperty),
    PowerToughness(terminals::PowerToughness),
    PowerToughnessModifier(terminals::PowerToughnessModifier),
    PlaneswalkerAbilityCost(terminals::PlaneswalkerAbilityCost),
    Object(object::Object),
    Zone(zone::Zone),
    Color(mtg_data::Color),
    KeywordAbility(mtg_data::KeywordAbility),
    Mana(mtg_data::Mana),
    CardType(mtg_data::CardType),
    CreatureType(mtg_data::CreatureType),
    EnchantmentType(mtg_data::EnchantmentType),
    LandType(mtg_data::LandType),
    PlaneswalkerType(mtg_data::PlaneswalkerType),
    BattleType(mtg_data::BattleType),
    ArtifactType(mtg_data::ArtifactType),
    SpellType(mtg_data::SpellType),
    Supertype(mtg_data::Supertype),
    ControlFlowToken(non_terminals::ControlFlowToken),
    TriggerAbilityMarker(non_terminals::TriggerAbilityMarker),
    TapUntapCost(non_terminals::TapUntapCost),
    EnglishKeywords(non_terminals::EnglishKeywords),
    SelfReferencing(non_terminals::SelfReferencing),
    NumberReference(non_terminals::NumberReference),
    NotOfAKind(non_terminals::NotOfAKind),
    ActionKeywords(non_terminals::ActionKeywords),
    DamageKind(non_terminals::DamageKind),
    PlayerActions(non_terminals::PlayerActions),
    VhyToSortLater(non_terminals::VhyToSortLater),
}

macro_rules! impl_into_token_kind {
    ($ty:path, $variant:tt) => {
        impl Into<TokenKind> for $ty {
            fn into(self) -> TokenKind {
                TokenKind::$variant(self)
            }
        }
    };
}

impl_into_token_kind!(terminals::Number, Number);
impl_into_token_kind!(terminals::Counter, Counter);
impl_into_token_kind!(terminals::CountSpecifier, CountSpecifier);
impl_into_token_kind!(terminals::ControlSpecifier, ControlSpecifier);
impl_into_token_kind!(terminals::OwnerSpecifier, OwnerSpecifier);
impl_into_token_kind!(terminals::Order, Order);
impl_into_token_kind!(terminals::Appartenance, Appartenance);
impl_into_token_kind!(terminals::CardActions, CardActions);
impl_into_token_kind!(terminals::PlayerSpecifier, PlayerSpecifier);
impl_into_token_kind!(terminals::PermanentState, PermanentState);
impl_into_token_kind!(terminals::PermanentProperty, PermanentProperty);
impl_into_token_kind!(terminals::SpellProperty, SpellProperty);
impl_into_token_kind!(terminals::Phase, Phase);
impl_into_token_kind!(terminals::Step, Step);
impl_into_token_kind!(terminals::PowerToughness, PowerToughness);
impl_into_token_kind!(terminals::PowerToughnessModifier, PowerToughnessModifier);
impl_into_token_kind!(terminals::PlaneswalkerAbilityCost, PlaneswalkerAbilityCost);
impl_into_token_kind!(object::Object, Object);
impl_into_token_kind!(zone::Zone, Zone);
impl_into_token_kind!(mtg_data::Color, Color);
impl_into_token_kind!(mtg_data::KeywordAbility, KeywordAbility);
impl_into_token_kind!(mtg_data::Mana, Mana);
impl_into_token_kind!(mtg_data::CardType, CardType);
impl_into_token_kind!(mtg_data::CreatureType, CreatureType);
impl_into_token_kind!(mtg_data::EnchantmentType, EnchantmentType);
impl_into_token_kind!(mtg_data::LandType, LandType);
impl_into_token_kind!(mtg_data::PlaneswalkerType, PlaneswalkerType);
impl_into_token_kind!(mtg_data::BattleType, BattleType);
impl_into_token_kind!(mtg_data::ArtifactType, ArtifactType);
impl_into_token_kind!(mtg_data::SpellType, SpellType);
impl_into_token_kind!(mtg_data::Supertype, Supertype);
impl_into_token_kind!(non_terminals::ControlFlowToken, ControlFlowToken);
impl_into_token_kind!(non_terminals::TriggerAbilityMarker, TriggerAbilityMarker);
impl_into_token_kind!(non_terminals::TapUntapCost, TapUntapCost);
impl_into_token_kind!(non_terminals::EnglishKeywords, EnglishKeywords);
impl_into_token_kind!(non_terminals::SelfReferencing, SelfReferencing);
impl_into_token_kind!(non_terminals::NumberReference, NumberReference);
impl_into_token_kind!(non_terminals::NotOfAKind, NotOfAKind);
impl_into_token_kind!(non_terminals::ActionKeywords, ActionKeywords);
impl_into_token_kind!(non_terminals::DamageKind, DamageKind);
impl_into_token_kind!(non_terminals::PlayerActions, PlayerActions);
impl_into_token_kind!(non_terminals::VhyToSortLater, VhyToSortLater);
