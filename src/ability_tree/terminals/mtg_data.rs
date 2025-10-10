use crate::ability_tree::terminals::Terminal;

fn from_str_singular_or_plural<T: std::str::FromStr>(source: &str) -> Option<T> {
    if let Ok(value) = T::from_str(source) {
        return Some(value);
    } else if source.ends_with('s') {
        if let Ok(value) = T::from_str(&source[..source.len() - 1]) {
            return Some(value);
        }
    }
    None
}

impl Terminal for mtg_data::KeywordAbility {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::Mana {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::CardType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::CreatureType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::EnchantmentType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::LandType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::PlaneswalkerType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::BattleType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::ArtifactType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::SpellType {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::Supertype {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::Color {
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}
