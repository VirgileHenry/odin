use crate::ability_tree::terminals;
use crate::ability_tree::terminals::Terminal;

#[derive(Debug, Clone, Copy)]
pub struct Token<'src> {
    kind: TokenKind,
    span: crate::lexer::span::Span<'src>,
}

impl<'src> Token<'src> {
    pub fn try_from_str(source: &'src str, offset: usize) -> Option<(Token<'src>, usize)> {
        let (kind, length) = {
            if let Some((kind, length)) = terminals::Number::try_from_str(source) {
                Some((TokenKind::Number(kind), length))
            } else if let Some((kind, length)) = terminals::Counter::try_from_str(source) {
                Some((TokenKind::Counter(kind), length))
            } else if let Some((kind, length)) = terminals::CountSpecifier::try_from_str(source) {
                Some((TokenKind::CountSpecifier(kind), length))
            } else if let Some((kind, length)) = terminals::ControlSpecifier::try_from_str(source) {
                Some((TokenKind::ControlSpecifier(kind), length))
            } else if let Some((kind, length)) = terminals::Appartenance::try_from_str(source) {
                Some((TokenKind::Appartenance(kind), length))
            } else if let Some((kind, length)) = terminals::Actions::try_from_str(source) {
                Some((TokenKind::Actions(kind), length))
            } else if let Some((kind, length)) = ControlFlowToken::try_from_str(source) {
                Some((TokenKind::ControlFlow(kind), length))
            } else {
                None
            }
        }?;
        Some((
            Token {
                kind,
                span: crate::lexer::span::Span {
                    start: offset,
                    length,
                    text: &source[0..length],
                },
            },
            length,
        ))
    }

    pub fn is_eof(&self) -> bool {
        match self.kind {
            TokenKind::ControlFlow(ControlFlowToken::EndOfInput) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Number(terminals::Number),
    Counter(terminals::Counter),
    CountSpecifier(terminals::CountSpecifier),
    ControlSpecifier(terminals::ControlSpecifier),
    Appartenance(terminals::Appartenance),
    Actions(terminals::Actions),
    ControlFlow(ControlFlowToken),
}

#[derive(Debug, Clone, Copy)]
pub enum ControlFlowToken {
    NewLine,
    Comma,
    Dot,
    Colons,
    EndOfInput,
}

impl ControlFlowToken {
    fn try_from_str(source: &str) -> Option<(Self, usize)> {
        match source.chars().next() {
            Some('\n') => Some((ControlFlowToken::NewLine, 1)),
            Some(',') => Some((ControlFlowToken::Comma, 1)),
            Some('.') => Some((ControlFlowToken::Dot, 1)),
            Some(':') => Some((ControlFlowToken::Colons, 1)),
            None => Some((ControlFlowToken::EndOfInput, 1)),
            _ => None,
        }
    }
}
