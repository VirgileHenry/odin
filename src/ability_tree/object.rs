#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Object {
    Creature,
    Card,
    Permanent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectReference {
    SelfReferencing,
    SpecifiedObj {
        amount: crate::ability_tree::terminals::CountSpecifier,
        object: Object,
        specifiers: Vec<ObjectSpecifier>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectSpecifier {
    Color(mtg_data::Color),
    Object(Object),
    Control(crate::ability_tree::terminals::ControlSpecifier),
}
