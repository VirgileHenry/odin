use mtg_data::{MtgColor, AbilityKeyword};

use crate::tokens::terminals::{
    actions::Actions,
    specifiers::{
        control::ControlSpecifier,
        count::CountSpecifier
    },
    counters::Counter,
    numbers::Number,
    objects::Object,
    Terminal,
    control_flow::ControlFlow, trigger_condition::TriggerConditionKW
};

use super::{
    AbilityTree,
    ability::{
        Ability,
        activated_ab::ActivatedAbility,
        spell_ab::SpellAbility, static_ab::StaticAbility,
        triggered_ab::TriggeredAbility
    },
    imperative::{Imperative, put::Put},
    object_reference::ObjectReference,
    suffix_specifier::ObjectSuffixSpecifier,
    prefix_specifier::ObjectPrefixSpecifier,
    statement::Statement,
    trigger_condition::TriggerCondition,
    english_keywords::EnglishKeywords
};


/// equivalent of token, but holding data for a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    Abilities(AbilityTree), // one or more ability
    Ability(Ability), // an ability
    Action(Actions), // an action an object can do
    ActivatedAb(ActivatedAbility), // activated ability
    ColorSpecifier(MtgColor), // any col spec (see Terminal::ColorSpec)
    ControlFlow(ControlFlow),
    ControlSpecifier(ControlSpecifier), // any contr spec (see Terminal::ControlSpec)
    Counter(Counter), // any counter (see Terminal::Counter)
    CountSpecifier(CountSpecifier), // count specifier 
    EndOfInput,
    EnglishKeywords(EnglishKeywords),
    Epsilon,
    Imperative(Imperative), // something a player must do
    Keyword(AbilityKeyword),
    Number(Number), // any number (see Terminal::Number)
    Object(Object), // represent an object
    ObjectReference(ObjectReference), // self referencing, or specified obj
    ObjSuffixSpec(ObjectSuffixSpecifier), // suffix specifier
    ObjPrefixSpec(ObjectPrefixSpecifier), // prefix specifier
    SpecifiedObj(Object, Vec<ObjectPrefixSpecifier>, Vec<ObjectSuffixSpecifier>), // object with pre/su fix spec
    SpellAb(SpellAbility), // spell ability
    Statement(Statement), // a statement ? maybe to rename
    StaticAb(StaticAbility), // static ability
    Text(AbilityTree), // Start Symbol
    TriggerCond(TriggerCondition), // trigger ability condition
    TriggeredAb(TriggeredAbility), // triggered ability
}

impl From<Terminal> for TreeNode {
    fn from(value: Terminal) -> TreeNode {
        match value {
            Terminal::ColorSpecifier(data) => TreeNode::ColorSpecifier(data),
            Terminal::ControlFlow(data) => TreeNode::ControlFlow(data),
            Terminal::ControlSpecifier(data) => TreeNode::ControlSpecifier(data),
            Terminal::Counter(data) => TreeNode::Counter(data),
            Terminal::CountSpecifier(data) => TreeNode::CountSpecifier(data),
            Terminal::EndOfInput => TreeNode::EndOfInput,
            Terminal::Epsilon => TreeNode::Epsilon,
            Terminal::ImperativeKW(data) => TreeNode::EnglishKeywords(data.into()),
            Terminal::Keyword(data) => TreeNode::Keyword(data),
            Terminal::Number(data) => TreeNode::Number(data),
            Terminal::Object(data) => TreeNode::Object(data),
            Terminal::SelfReferencing => TreeNode::ObjectReference(ObjectReference::SelfReferencing),
            Terminal::TerminalAction(data) => TreeNode::Action(data),
            Terminal::TriggerCondKW(data) => TreeNode::EnglishKeywords(data.into()),
        }
    }
}

impl TryFrom<&[TreeNode]> for TreeNode {
    type Error = ();
    fn try_from(value: &[TreeNode]) -> Result<Self, Self::Error> {
        // this is where the magic happens. All the magic parsing rules encoded in this from impl.
        match value {
            // all start rules
            [TreeNode::Abilities(ab), TreeNode::EndOfInput] => Ok(TreeNode::Text(ab.clone())),
            [TreeNode::Epsilon] => Ok(TreeNode::Text(AbilityTree { abilities: vec![] })),
            // abilities to ability
            [TreeNode::Ability(ab)] => Ok(TreeNode::Abilities(AbilityTree { abilities: vec![ab.clone()] })),
            [TreeNode::Ability(added), TreeNode::ControlFlow(cf), TreeNode::Abilities(abs)] => match cf {
                ControlFlow::NewLine => {
                    let mut new_abs = abs.clone();
                    // put it first to keep order
                    new_abs.abilities.insert(0, added.clone());
                    Ok(TreeNode::Abilities(new_abs))
                },
                _ => Err(()),
            },
            // ability to all types of abilities
            [TreeNode::TriggeredAb(ab)] => Ok(TreeNode::Ability(Ability::Triggered(ab.clone()))),
            [TreeNode::ActivatedAb(ab)] => Ok(TreeNode::Ability(Ability::Activated(ab.clone()))),
            [TreeNode::StaticAb(ab)] => Ok(TreeNode::Ability(Ability::Static(ab.clone()))),
            [TreeNode::SpellAb(ab)] => Ok(TreeNode::Ability(Ability::Spell(ab.clone()))),
            // Trigger ab rule
            [TreeNode::TriggerCond(cond), TreeNode::ControlFlow(cf), TreeNode::Statement(stmt)] => match cf {
                ControlFlow::Comma => Ok(TreeNode::TriggeredAb(TriggeredAbility::TriggerAbility(cond.clone(), stmt.clone()))),
                _ => Err(()),
            },
            // trigger cond
            [TreeNode::EnglishKeywords(kw), TreeNode::ObjectReference(obj_ref), TreeNode::Action(action)] => match kw {
                EnglishKeywords::When => Ok(TreeNode::TriggerCond(TriggerCondition::ObjectDoesAction(TriggerConditionKW::When, obj_ref.clone(), *action))),
                _ => Err(()),
            },
            // statements from imperative
            [TreeNode::Imperative(imp)] => Ok(TreeNode::Statement(Statement::Imperative(imp.clone()))),
            // imperatives possibilities : 
            // put
            [TreeNode::EnglishKeywords(kw_1), TreeNode::Number(num), TreeNode::Counter(counter),
                TreeNode::EnglishKeywords(kw_2), TreeNode::ObjectReference(obj_ref),
                TreeNode::ControlFlow(cf)] => match (kw_1, kw_2, cf)  {
                (EnglishKeywords::Put, EnglishKeywords::On, ControlFlow::Dot) => Ok(TreeNode::Imperative(Imperative::Put(Put::Counters(*num, *counter, obj_ref.clone())))),
                _ => Err(()),
            },
            // object references
            [TreeNode::CountSpecifier(count), TreeNode::SpecifiedObj(obj, pre, suf)]
                => Ok(TreeNode::ObjectReference(ObjectReference::SpecifiedObj(*count, *obj, pre.clone(), suf.clone()))),
            // specified object
            [TreeNode::Object(obj)] => Ok(TreeNode::SpecifiedObj(*obj, vec![], vec![])),
            [TreeNode::ObjPrefixSpec(pre), TreeNode::SpecifiedObj(obj, prefs, sufs)]
                => Ok(TreeNode::SpecifiedObj(*obj, {let mut n = vec![*pre]; n.append(&mut prefs.clone()); n}, sufs.clone())),
            [TreeNode::SpecifiedObj(obj, prefs, sufs), TreeNode::ObjSuffixSpec(suf)]
                => Ok(TreeNode::SpecifiedObj(*obj, prefs.clone(), {let mut n = vec![*suf]; n.append(&mut sufs.clone()); n})),
            // prefix specifiers
            [TreeNode::ColorSpecifier(color)] => Ok(TreeNode::ObjPrefixSpec(ObjectPrefixSpecifier::Color(*color))),
            // suffix specifier
            [TreeNode::ControlSpecifier(control)] => Ok(TreeNode::ObjSuffixSpec(ObjectSuffixSpecifier::Control(*control))),
            // keyword abilities can be static or triggered abilities
            [TreeNode::Keyword(kw)] => match kw {
                AbilityKeyword::Flying => Ok(TreeNode::StaticAb(StaticAbility::CommonStaticAbility(AbilityKeyword::Flying))),
                AbilityKeyword::Mentor => Ok(TreeNode::TriggeredAb(TriggeredAbility::CommonTriggeredAbilities(AbilityKeyword::Mentor))),
            }
            _ => Err(()),
        }
    }
}

