/// All the layouts of Magic: The Gathering for playable cards.
/// This is a subset of this [Layout](crate::card::printed::layout::Layout).
pub enum Layout {
    Normal {
        mana_cost: crate::card::ManaCost,
        card_type: super::CardType,
        abilities: crate::ability_tree::AbilityTree,
    },
    Split {},
    Flip {},
    Transform {},
    ModalDfc {},
    Meld {},
    Leveler {},
    Class {},
    Case {},
    Saga {},
    Adventure {},
    Mutate {},
    Prototype {},
    Battle {},
    Planar {},
    Scheme {},
    Vanguard {},
    Token {},
    DoubleFaced {},
    Emblem {},
}
