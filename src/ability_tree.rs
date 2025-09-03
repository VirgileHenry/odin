pub mod ability;
pub mod imperative;
pub mod object;
pub mod statement;
pub mod terminals;
pub mod zone;

/// One or more abilities.
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityTree {
    pub abilities: Vec<ability::Ability>,
}

impl AbilityTree {
    pub fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        writeln!(out, "Abilities:")?;

        for ability in self.abilities.iter() {
            ability.display(out)?;
        }

        Ok(())
    }
}

pub trait AbilityTreeImpl {
    fn display<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()>;
}

pub fn example() -> AbilityTree {
    AbilityTree {
        abilities: vec![ability::Ability::Triggered(
            ability::triggered::TriggeredAbility {
                condition: ability::triggered::trigger_cond::TriggerCondition::ObjectDoesAction {
                    object: object::ObjectReference::SelfReferencing,
                    action: terminals::Actions::Dies,
                },
                effect: statement::Statement::Imperative(imperative::Imperative::Put {
                    amount: terminals::Number::A,
                    of: terminals::Counter::PlusOne,
                    on: object::ObjectReference::SpecifiedObj {
                        amount: terminals::CountSpecifier::Each,
                        object: object::Object::Creature,
                        specifiers: vec![
                            object::ObjectSpecifier::Control(
                                terminals::ControlSpecifier::YouControl,
                            ),
                            object::ObjectSpecifier::Color(mtg_data::Color::White),
                        ],
                    },
                }),
            },
        )],
    }
}
