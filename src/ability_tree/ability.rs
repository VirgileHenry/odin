pub mod activated;
pub mod keyword;
pub mod spell;
pub mod statik;
pub mod triggered;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ability {
    Keyword(keyword::KeywordAbility),
    Activated(activated::ActivatedAbility),
    Spell(spell::SpellAbility),
    Static(statik::StaticAbility),
    Triggered(triggered::TriggeredAbility),
}

impl crate::ability_tree::AbilityTreeImpl for Ability {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        match self {
            Ability::Activated(activated) => activated.display(out)?,
            Ability::Spell(spell) => spell.display(out)?,
            Ability::Static(statik) => statik.display(out)?,
            Ability::Triggered(triggered) => triggered.display(out)?,
            Ability::Keyword(keyword) => keyword.display(out)?,
        }
        Ok(())
    }
}
