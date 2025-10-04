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

impl crate::ability_tree::terminals::Terminal for Object {
    fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "creature" => Some(Object::Creature),
            "card" => Some(Object::Card),
            "permanent" => Some(Object::Permanent),
            _ => None,
        }
    }
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

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectReference::SelfReferencing => write!(out, "Self Referencing (~)"),
            ObjectReference::SpecifiedObj {
                amount,
                object,
                specifiers,
            } => {
                write!(out, "Specified Object:")?;
                out.push_inter_branch()?;
                write!(out, "Amount:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "Object(s):")?;
                out.push_final_branch()?;
                write!(out, "{object}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "Specifier(s):")?;
                for specifier in specifiers.iter().take(specifiers.len().saturating_sub(1)) {
                    out.push_inter_branch()?;
                    specifier.display(out)?;
                    out.pop_branch();
                }
                if let Some(specifier) = specifiers.last() {
                    out.push_final_branch()?;
                    specifier.display(out)?;
                    out.pop_branch();
                }
                out.pop_branch();
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
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifier::Color(color) => write!(out, "Color Specifier: {color}"),
            ObjectSpecifier::Object(object) => write!(out, "Object Specifier: {object}"),
            ObjectSpecifier::Control(control) => write!(out, "Control Specifier: {control}"),
        }
    }
}
