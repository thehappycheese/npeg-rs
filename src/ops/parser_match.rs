#[derive(Debug)]
pub struct ParserMatch<'a> {
    pub start: usize,
    pub end: usize,
    pub label: Option<&'a str>,
}
impl<'a> ParserMatch<'a> {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}