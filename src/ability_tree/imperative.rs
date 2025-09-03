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

impl crate::ability_tree::AbilityTreeImpl for Imperative {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self {
            Imperative::Put { amount, of, on } => {
                writeln!(out, "Put:")?;
                writeln!(out, "Number: {amount}")?;
                writeln!(out, "Of: {of}")?;
                writeln!(out, "On:")?;
                on.display(out)
            }
            Imperative::Return { object, from, to } => {
                writeln!(out, "Return:")?;
                writeln!(out, "Object:")?;
                object.display(out)?;
                writeln!(out, "From:")?;
                from.display(out)?;
                writeln!(out, "To:")?;
                to.display(out)
            }
            Imperative::DealsDamage { dealer, amount, to } => {
                writeln!(out, "Deals Damage:")?;
                writeln!(out, "Object:")?;
                dealer.display(out)?;
                writeln!(out, "Amount: {amount}")?;
                writeln!(out, "To:")?;
                to.display(out)
            }
            Imperative::Sacrifice { object } => {
                writeln!(out, "Sacrifice:")?;
                writeln!(out, "Object:")?;
                object.display(out)
            }
        }
    }
}
