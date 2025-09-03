#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StaticAbility {}

impl crate::ability_tree::AbilityTreeImpl for StaticAbility {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "Todo!")
    }
}
