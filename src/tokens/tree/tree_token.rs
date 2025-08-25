use mtg_data::{Color, KeywordAbility, Mana};

use crate::tokens::terminals::{
    actions::Actions,
    specifiers::{
        control::ControlSpecifier,
        count::CountSpecifier,
        appartenace::AppartenanceSpecifier,
    },
    counters::Counter,
    numbers::Number,
    objects::Object,
    Terminal,
    control_flow::ControlFlow, zone::Zone,
};

use super::{
    AbilityTree,
    ability::{
        Ability,
        activated_ab::ActivatedAbility,
        spell_ab::SpellAbility, static_ab::StaticAbility,
        triggered_ab::TriggeredAbility
    },
    imperative::Imperative,
    object_reference::ObjectReference,
    suffix_specifier::ObjectSuffixSpecifier,
    prefix_specifier::{ObjectPrefixSpecifier, ZonePrefixSpecifier},
    statement::Statement,
    trigger_condition::TriggerCondition,
    english_keywords::EnglishKeywords, zone_reference::ZoneReference, cost::Cost
};


/// equivalent of token, but holding data for a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    Abilities(AbilityTree), // one or more ability
    Ability(Ability), // an ability
    Action(Actions), // an action an object can do
    ActivatedAb(ActivatedAbility), // activated ability
    AppartenanceSpecifier(AppartenanceSpecifier), // any appartenance spec (your, an opponent's)
    ColorSpecifier(Color), // any col spec (see Terminal::ColorSpec)
    ControlFlow(ControlFlow),
    ControlSpecifier(ControlSpecifier), // any contr spec (see Terminal::ControlSpec)
    Cost(Cost),
    Costs(Vec<Cost>),
    Counter(Counter), // any counter (see Terminal::Counter)
    CountSpecifier(CountSpecifier), // count specifier 
    EndOfInput,
    EnglishKeywords(EnglishKeywords),
    Imperative(Imperative), // something a player must do
    Keyword(KeywordAbility),
    Number(Number), // any number (see Terminal::Number)
    Object(Object), // represent an object
    ObjectReference(ObjectReference), // self referencing, or specified obj
    ObjSuffixSpec(ObjectSuffixSpecifier), // suffix specifier
    ObjPrefixSpec(ObjectPrefixSpecifier), // prefix specifier
    SpecifiedObj(Object, Vec<ObjectPrefixSpecifier>, Vec<ObjectSuffixSpecifier>), // object with pre/su fix spec
    SpecifiedZone(Zone, Vec<ZonePrefixSpecifier>),
    SpellAb(SpellAbility), // spell ability
    Statement(Statement), // a statement ? maybe to rename
    StaticAb(StaticAbility), // static ability
    Text(AbilityTree), // Start Symbol
    TriggerCond(TriggerCondition), // trigger ability condition
    TriggeredAb(TriggeredAbility), // triggered ability
    Zone(Zone),
    ZoneReference(ZoneReference),
}

impl From<Terminal> for TreeNode {
    fn from(value: Terminal) -> TreeNode {
        match value {
            Terminal::AppartenanceSpecifier(data) => TreeNode::AppartenanceSpecifier(data),
            Terminal::ColorSpecifier(data) => TreeNode::ColorSpecifier(data),
            Terminal::ControlFlow(data) => TreeNode::ControlFlow(data),
            Terminal::ControlSpecifier(data) => TreeNode::ControlSpecifier(data),
            Terminal::Counter(data) => TreeNode::Counter(data),
            Terminal::CountSpecifier(data) => TreeNode::CountSpecifier(data),
            Terminal::EndOfInput => TreeNode::EndOfInput,
            Terminal::ImperativeKW(data) => TreeNode::EnglishKeywords(data.into()),
            Terminal::Keyword(data) => TreeNode::Keyword(data),
            Terminal::Number(data) => TreeNode::Number(data),
            Terminal::Object(data) => TreeNode::Object(data),
            Terminal::SelfReferencing => TreeNode::ObjectReference(ObjectReference::SelfReferencing),
            Terminal::TerminalAction(data) => TreeNode::Action(data),
            Terminal::TriggerCondKW(data) => TreeNode::EnglishKeywords(data.into()),
            Terminal::Zone(data) => TreeNode::Zone(data),
            Terminal::Mana(data) => TreeNode::Cost(Cost::ManaCost(data)),
            Terminal::TapCost => TreeNode::Cost(Cost::TapCost),
        }
    }
}
