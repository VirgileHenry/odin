#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellAbility {
    effect: crate::ability_tree::statement::Statement,
}

impl crate::ability_tree::AbilityTreeImpl for SpellAbility {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        self.effect.display(out)
    }
}
