#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'source> {
    pub start: usize,
    pub length: usize,
    pub text: &'source str,
}
