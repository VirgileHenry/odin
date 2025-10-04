#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    keyword: mtg_data::KeywordAbility,
}

impl crate::ability_tree::AbilityTreeImpl for KeywordAbility {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "Keyword: {}", self.keyword)?;
        Ok(())
    }
}
