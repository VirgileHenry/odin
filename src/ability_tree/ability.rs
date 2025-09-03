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

impl crate::ability_tree::AbilityTreeImpl for Ability {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            Ability::Activated(activated) => {
                writeln!(out, "Activated Ability:")?;
                activated.display(out)
            }
            Ability::Spell(spell) => {
                writeln!(out, "Spell Ability:")?;
                spell.display(out)
            }
            Ability::Static(statik) => {
                writeln!(out, "Static Ability:")?;
                statik.display(out)
            }
            Ability::Triggered(triggered) => {
                writeln!(out, "Triggered Ability:")?;
                triggered.display(out)
            }
        }
    }
}
