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
            &gen_parse_func(terminals::Appartenance::try_from_str),
            &gen_parse_func(terminals::CardActions::try_from_str),
            &gen_parse_func(terminals::PlayerActions::try_from_str),
            &gen_parse_func(terminals::PlayerSpecifier::try_from_str),
            &gen_parse_func(object::Object::try_from_str),
            &gen_parse_func(zone::Zone::try_from_str),
            &gen_parse_func(mtg_data::KeywordAbility::try_from_str),
            &gen_parse_func(mtg_data::Mana::try_from_str),
            &gen_parse_func(non_terminals::ControlFlowToken::try_from_str),
            &gen_parse_func(non_terminals::TriggerAbilityMarker::try_from_str),
            &gen_parse_func(non_terminals::TapUntapCost::try_from_str),
            &gen_parse_func(non_terminals::EnglishKeywords::try_from_str),
            &gen_parse_func(non_terminals::SelfReferencing::try_from_str),
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
    Appartenance(terminals::Appartenance),
    CardActions(terminals::CardActions),
    PlayerActions(terminals::PlayerActions),
    PlayerSpecifier(terminals::PlayerSpecifier),
    Object(object::Object),
    Zone(zone::Zone),
    KeywordAbility(mtg_data::KeywordAbility),
    ManaCost(mtg_data::Mana),
    ControlFlowToken(non_terminals::ControlFlowToken),
    TriggerAbilityMarker(non_terminals::TriggerAbilityMarker),
    TapUntapCost(non_terminals::TapUntapCost),
    EnglishKeywords(non_terminals::EnglishKeywords),
    SelfReferencing(non_terminals::SelfReferencing),
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
impl_into_token_kind!(terminals::Appartenance, Appartenance);
impl_into_token_kind!(terminals::CardActions, CardActions);
impl_into_token_kind!(terminals::PlayerActions, PlayerActions);
impl_into_token_kind!(terminals::PlayerSpecifier, PlayerSpecifier);
impl_into_token_kind!(object::Object, Object);
impl_into_token_kind!(zone::Zone, Zone);
impl_into_token_kind!(mtg_data::KeywordAbility, KeywordAbility);
impl_into_token_kind!(mtg_data::Mana, ManaCost);
impl_into_token_kind!(non_terminals::ControlFlowToken, ControlFlowToken);
impl_into_token_kind!(non_terminals::TriggerAbilityMarker, TriggerAbilityMarker);
impl_into_token_kind!(non_terminals::TapUntapCost, TapUntapCost);
impl_into_token_kind!(non_terminals::EnglishKeywords, EnglishKeywords);
impl_into_token_kind!(non_terminals::SelfReferencing, SelfReferencing);
