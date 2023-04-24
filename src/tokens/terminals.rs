use std::{fmt::{Display}, str::FromStr};

use mtg_data::{MtgColor, AbilityKeyword};
use self::{
    actions::Actions,
    control_flow::ControlFlow,
    counters::Counter,
    numbers::Number,
    specifiers::{
        control::ControlSpecifier,
        count::CountSpecifier,
    },
    objects::Object,
    trigger_condition::TriggerConditionKW,
    imperatives::ImperativeKW,
};

/// An enum for all terminal nodes of our grammar. Also referenced as a token for lexing.
/// We wrap them up in categories to easy a little parsing and recognition.
/// Technically, these are non terminal that all gives the terminals they hold.
/// Therfore, some of them have a non terminal equivalent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Terminal {
    ColorSpecifier(MtgColor),
    ControlFlow(ControlFlow),
    ControlSpecifier(ControlSpecifier),
    Counter(Counter),
    CountSpecifier(CountSpecifier),
    EndOfInput,
    Epsilon, // used for parsing, but never constructed from lexing.
    ImperativeKW(ImperativeKW),
    Keyword(AbilityKeyword),
    Number(Number),
    Object(Object),
    SelfReferencing,
    TerminalAction(Actions),
    TriggerCondKW(TriggerConditionKW),
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{self:?}]")
    }
}

impl FromStr for Terminal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // try each variant from string.
        // feel like this could be written in a derive macro, but it's lot of work. for this only use case.
        match ControlFlow::from_str(s) {
            Ok(cf) => return Ok(Terminal::ControlFlow(cf)),
            Err(_) => {},
        }
        match AbilityKeyword::from_str(s) {
            Ok(kw) => return Ok(Terminal::Keyword(kw)),
            Err(_) => {},
        }
        match MtgColor::from_str(s) {
            Ok(color) => return Ok(Terminal::ColorSpecifier(color)),
            Err(_) => {},
        }
        match ControlSpecifier::from_str(s) {
            Ok(spec) => return Ok(Terminal::ControlSpecifier(spec)),
            Err(_) => {},
        }
        match Counter::from_str(s) {
            Ok(counter) => return Ok(Terminal::Counter(counter)),
            Err(_) => {},
        }
        match Object::from_str(s) {
            Ok(obj) => return Ok(Terminal::Object(obj)),
            Err(_) => {},
        }
        match Number::from_str(s) {
            Ok(n) => return Ok(Terminal::Number(n)),
            Err(_) => {},
        }
        match CountSpecifier::from_str(s) {
            Ok(spec) => return Ok(Terminal::CountSpecifier(spec)),
            Err(_) => {},
        }
        match Actions::from_str(s) {
            Ok(action) => return Ok(Terminal::TerminalAction(action)),
            Err(_) => {},
        }
        match TriggerConditionKW::from_str(s) {
            Ok(cond) => return Ok(Terminal::TriggerCondKW(cond)),
            Err(_) => {},
        }
        match ImperativeKW::from_str(s) {
            Ok(imperative) => return Ok(Terminal::ImperativeKW(imperative)),
            Err(_) => {},
        }

        // finally, our own terminal choices
        match s {
            "~" => return Ok(Terminal::SelfReferencing),
            "$" => return Ok(Terminal::EndOfInput),
            _ => {},
        }

        Err(())
    }
}


pub mod actions;
pub mod control_flow;
pub mod counters;
pub mod imperatives;
pub mod numbers;
pub mod objects;
pub mod specifiers;
pub mod trigger_condition;