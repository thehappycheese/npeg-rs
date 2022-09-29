#[derive(Debug)]
pub struct ParserMatch {
    pub start: usize,
    pub end: usize,
    pub label: Option<String>,
    pub children: Vec<ParserMatch>,
}
impl ParserMatch {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn get_text<'a> (&self, full_text:&'a str) -> &'a str{
        full_text[self.start..self.end].into()
    }

    pub fn with_label(self, new_label:&str)->Self{
        Self{
            start   : self.start,
            end     : self.end,
            label   : Some(new_label.into()),
            children: self.children,
        }
    }
}

#[cfg(tests)]
mod tests {

    #[test]
    fn match_gets_correct_substring() {
        let m = ParserMatch{
            start    : 0,
            end      : 1,
            label    : None,
            children : vec![],
        };
        let full_text = "0123456789";
        assert!(m.get_text(full_text), "0");
    }

}