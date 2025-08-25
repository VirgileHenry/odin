#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Imperative {
    Put {
        amount: crate::ability_tree::terminals::Number,
        of: crate::ability_tree::terminals::Counter,
        on: crate::ability_tree::object::ObjectReference,
    },
    Return {
        object: crate::ability_tree::object::ObjectReference,
        from: crate::ability_tree::zone::ZoneReference,
        to: crate::ability_tree::zone::ZoneReference,
    },
    DealsDamage {
        dealer: crate::ability_tree::object::ObjectReference,
        amount: crate::ability_tree::terminals::Number,
        to: crate::ability_tree::object::ObjectReference,
    },
    Sacrifice {
        object: crate::ability_tree::object::ObjectReference,
    },
}
