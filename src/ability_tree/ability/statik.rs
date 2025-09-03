#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StaticAbility {}

impl crate::ability_tree::AbilityTreeImpl for StaticAbility {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Todo!")
    }
}
