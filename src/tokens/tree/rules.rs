use mtg_data::KeywordAbility;

use super::{tree_token::TreeNode, ability::{spell_ab::SpellAbility, activated_ab::ActivatedAbility}, zone_reference::ZoneReference, cost::Cost};

use crate::tokens::terminals::{
    control_flow::ControlFlow,
    trigger_condition::TriggerConditionKW, zone::Zone
};

use super::{
    AbilityTree,
    ability::{
        Ability,
        static_ab::StaticAbility,
        triggered_ab::TriggeredAbility
    },
    imperative::Imperative,
    object_reference::ObjectReference,
    suffix_specifier::ObjectSuffixSpecifier,
    prefix_specifier::ObjectPrefixSpecifier,
    statement::Statement,
    trigger_condition::TriggerCondition,
    english_keywords::EnglishKeywords
};


pub fn try_rule(value: &[TreeNode]) -> Result<Vec<TreeNode>, ()> {
    // this is where the magic happens. All the magic parsing rules encoded in this from impl.
    match value {
        // all start rules
        [TreeNode::Abilities(ab), TreeNode::EndOfInput] => Ok(vec![TreeNode::Text(ab.clone())]),
        [TreeNode::EndOfInput] => Ok(vec![TreeNode::Text(AbilityTree { abilities: vec![] })]),
        // abilities to ability
        [TreeNode::Ability(ab)] => Ok(vec![TreeNode::Abilities(AbilityTree { abilities: vec![ab.clone()] })]),
        [TreeNode::Ability(added), TreeNode::ControlFlow(cf), TreeNode::Abilities(abs)] => match cf {
            ControlFlow::NewLine => {
                let mut new_abs = abs.clone();
                // put it first to keep order
                new_abs.abilities.insert(0, added.clone());
                Ok(vec![TreeNode::Abilities(new_abs)])
            },
            _ => Err(()),
        },


        // ability to all types of abilities
        [TreeNode::TriggeredAb(ab)] => Ok(vec![TreeNode::Ability(Ability::Triggered(ab.clone()))]),
        [TreeNode::ActivatedAb(ab)] => Ok(vec![TreeNode::Ability(Ability::Activated(ab.clone()))]),
        [TreeNode::StaticAb(ab)] => Ok(vec![TreeNode::Ability(Ability::Static(ab.clone()))]),
        [TreeNode::SpellAb(ab)] => Ok(vec![TreeNode::Ability(Ability::Spell(ab.clone()))]),
        // now let's go over abilities to statements
        // Trigger ab rule
        [TreeNode::TriggerCond(cond), TreeNode::ControlFlow(cf), TreeNode::Statement(stmt)] => match cf {
            ControlFlow::Comma => Ok(vec![TreeNode::TriggeredAb(TriggeredAbility::TriggerAbility(cond.clone(), stmt.clone()))]),
            _ => Err(()),
        },

        // statement to ab
        [TreeNode::Statement(stmt)] => Ok(vec![TreeNode::SpellAb(SpellAbility::Statement(stmt.clone()))]),
        // trigger cond
        [TreeNode::EnglishKeywords(kw), TreeNode::ObjectReference(obj_ref), TreeNode::Action(action)] => match kw {
            EnglishKeywords::When => Ok(vec![TreeNode::TriggerCond(TriggerCondition::ObjectDoesAction(TriggerConditionKW::When, obj_ref.clone(), *action))]),
            _ => Err(()),
        },

        // activated ab
        [TreeNode::Costs(costs), TreeNode::ControlFlow(cf), TreeNode::Statement(stmt)] => match cf {
            ControlFlow::Colons => Ok(vec![TreeNode::ActivatedAb(ActivatedAbility::CostStatement(costs.clone(), stmt.clone()))]),
            _ => Err(()),
        }

        // statements from imperative
        [TreeNode::Imperative(imp)] => Ok(vec![
            TreeNode::Statement(Statement::Imperative(imp.clone())),
            TreeNode::Cost(Cost::Imperative(imp.clone())),
        ]),
        
        // cost creation:
        [TreeNode::Cost(cost)] => Ok(vec![TreeNode::Costs(vec![cost.clone()])]),
        [TreeNode::Cost(cost), TreeNode::ControlFlow(cf), TreeNode::Costs(costs)] => match cf {
            ControlFlow::Comma => Ok(vec![TreeNode::Costs({let mut v = costs.clone(); v.insert(0, cost.clone()); v})]),
            _ => Err(())
        }

        // imperatives possibilities : 
        
        
        // put
        [TreeNode::EnglishKeywords(kw_1), TreeNode::Number(num), TreeNode::Counter(counter),
            TreeNode::EnglishKeywords(kw_2), TreeNode::ObjectReference(obj_ref),
            TreeNode::ControlFlow(cf)] => match (kw_1, kw_2, cf)  {
            (EnglishKeywords::Put, EnglishKeywords::On, ControlFlow::Dot) => Ok(vec![TreeNode::Imperative(Imperative::Put(*num, *counter, obj_ref.clone()))]),
            _ => Err(()),
        },
        // return 
        [TreeNode::EnglishKeywords(kw_1), TreeNode::ObjectReference(obj), TreeNode::EnglishKeywords(kw_2),
            TreeNode::ZoneReference(zone1), TreeNode::EnglishKeywords(kw3), TreeNode::ZoneReference(zone2),
            TreeNode::ControlFlow(cf)] => match (kw_1, kw_2, kw3, cf)  {
            (EnglishKeywords::Return, EnglishKeywords::From, EnglishKeywords::To, ControlFlow::Dot) => Ok(vec![TreeNode::Imperative(Imperative::Return(obj.clone(), zone1.clone(), zone2.clone()))]),
            _ => Err(()),
        },
        // deals damage
        [TreeNode::ObjectReference(obj_1), TreeNode::EnglishKeywords(kw_1), TreeNode::Number(num), 
            TreeNode::EnglishKeywords(kw_2), TreeNode::EnglishKeywords(kw_3), TreeNode::ObjectReference(obj_2),
            TreeNode::ControlFlow(cf)] => match (kw_1, kw_2, kw_3, cf) {
            (EnglishKeywords::Deals, EnglishKeywords::Damage, EnglishKeywords::To, ControlFlow::Dot) => Ok(vec![TreeNode::Imperative(Imperative::DealsDamage(obj_1.clone(), *num, obj_2.clone()))]),
            _ => Err(()),
        },
        // sacrifice
        [TreeNode::EnglishKeywords(kw), TreeNode::ObjectReference(obj)] => match kw {
            EnglishKeywords::Sacrifice => Ok(vec![TreeNode::Imperative(Imperative::Sacrifice(obj.clone()))]),
            _ => Err(()),
        }


        // object references
        [TreeNode::CountSpecifier(count), TreeNode::SpecifiedObj(obj, pre, suf)]
            => Ok(vec![TreeNode::ObjectReference(ObjectReference::SpecifiedObj(*count, *obj, pre.clone(), suf.clone()))]),
        // specified object
        [TreeNode::Object(obj)] => Ok(vec![TreeNode::SpecifiedObj(*obj, vec![], vec![])]),
        [TreeNode::ObjPrefixSpec(pre), TreeNode::SpecifiedObj(obj, prefs, sufs)]
            => Ok(vec![TreeNode::SpecifiedObj(*obj, {let mut n = vec![*pre]; n.append(&mut prefs.clone()); n}, sufs.clone())]),
        [TreeNode::SpecifiedObj(obj, prefs, sufs), TreeNode::ObjSuffixSpec(suf)]
            => Ok(vec![TreeNode::SpecifiedObj(*obj, prefs.clone(), {let mut n = vec![*suf]; n.append(&mut sufs.clone()); n})]),
        
        
        
        // zones
        // special case of the battlefield, only not belonged zone
        [TreeNode::Zone(zone)] => match zone {
            Zone::Battlefield => Ok(vec![TreeNode::ZoneReference(ZoneReference::TheBattlefield)]),
            _ => Err(()),
        },
        [TreeNode::AppartenanceSpecifier(app), TreeNode::Zone(zone)] => Ok(vec![TreeNode::ZoneReference(ZoneReference::OwnedZone(*zone, *app))]),
        
        
        // prefix specifiers
        [TreeNode::ColorSpecifier(color)] => Ok(vec![TreeNode::ObjPrefixSpec(ObjectPrefixSpecifier::Color(*color))]),
        // objects can be specifier (permanent card, artifact creature)
        [TreeNode::Object(spec), TreeNode::Object(obj)] => Ok(vec![TreeNode::SpecifiedObj(*obj, vec![ObjectPrefixSpecifier::Object(*spec)], vec![])]),
        // suffix specifier
        [TreeNode::ControlSpecifier(control)] => Ok(vec![TreeNode::ObjSuffixSpec(ObjectSuffixSpecifier::Control(*control))]),
        
        
        
        
        
        // keyword abilities can be static or triggered abilities
        [TreeNode::Keyword(kw)] => match kw {
            KeywordAbility::Flying => Ok(vec![TreeNode::StaticAb(StaticAbility::CommonStaticAbility(KeywordAbility::Flying))]),
            KeywordAbility::Mentor => Ok(vec![TreeNode::TriggeredAb(TriggeredAbility::CommonTriggeredAbilities(KeywordAbility::Mentor))]),
            KeywordAbility::Rebound => Ok(vec![TreeNode::SpellAb(SpellAbility::CommonSpellAbility(KeywordAbility::Rebound))]),
            _ => panic!("Unsupported in rules: {kw:?}")
        }
        _ => Err(()),
    }
}