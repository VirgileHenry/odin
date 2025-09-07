#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'src> {
    pub start: usize,
    pub length: usize,
    pub text: &'src str,
}
