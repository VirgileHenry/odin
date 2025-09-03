#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Object {
    Creature,
    Card,
    Permanent,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Creature => write!(f, "Creature"),
            Object::Card => write!(f, "Card"),
            Object::Permanent => write!(f, "Permanent"),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for Object {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectReference {
    SelfReferencing,
    SpecifiedObj {
        amount: crate::ability_tree::terminals::CountSpecifier,
        object: Object,
        specifiers: Vec<ObjectSpecifier>,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            ObjectReference::SelfReferencing => write!(out, "~ (Self Referencing)"),
            ObjectReference::SpecifiedObj {
                amount,
                object,
                specifiers,
            } => {
                writeln!(out, "Specified Object:")?;
                writeln!(out, "Amount: {amount}")?;
                writeln!(out, "Objects: {object}")?;
                writeln!(out, "Specifiers:")?;
                for specifier in specifiers.iter() {
                    specifier.display(out)?;
                    writeln!(out, "")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectSpecifier {
    Color(mtg_data::Color),
    Object(Object),
    Control(crate::ability_tree::terminals::ControlSpecifier),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifier {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            ObjectSpecifier::Color(color) => write!(out, "Color Specifier: {color}"),
            ObjectSpecifier::Object(object) => write!(out, "Object Specifier: {object}"),
            ObjectSpecifier::Control(control) => write!(out, "Control Specifier: {control}"),
        }
    }
}
