pub mod activated;
pub mod spell;
pub mod statik;
pub mod triggered;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ability {
    Activated(activated::ActivatedAbility),
    Spell(spell::SpellAbility),
    Static(statik::StaticAbility),
    Triggered(triggered::TriggeredAbility),
}
