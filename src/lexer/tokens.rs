use crate::ability_tree::terminals;

pub struct Token<'source> {
    kind: TokenKind,
    span: crate::lexer::span::Span<'source>,
}

pub enum TokenKind {
    Number(terminals::Number),
    Counter(terminals::Counter),
    CountSpecifier(terminals::CountSpecifier),
    ControlSpecifier(terminals::ControlSpecifier),
    AppartenanceSpecifier(terminals::AppartenanceSpecifier),
    Actions(terminals::Actions),
    ControlFlow(ControlFlowToken),
}

pub enum ControlFlowToken {
    NewLine,
    Comma,
    Dot,
    Colons,
    EndOfInput,
}
