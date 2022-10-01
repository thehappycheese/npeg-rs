///////////////////////
///   ParserMatch   ///
///////////////////////

use std::rc::Rc;

#[derive(Debug)]
pub struct ParserMatch {
    start_position: usize,
    end_position: usize,
    label: Option<String>,
    children: Vec<Rc<ParserMatch>>,
}
impl ParserMatch {
    pub fn new(start_position: usize, end_position: usize, label: Option<String>, children: Vec<Rc<Self>>)->Rc<Self>{
        Rc::new(ParserMatch {
            start_position,
            end_position,
            label,
            children,
        })
    }
    pub fn len(&self) -> usize {
        self.end_position - self.start_position
    }
    pub fn get_text<'a> (&self, full_text:&'a str) -> &'a str{
        full_text[self.start_position..self.end_position].into()
    }

    pub fn with_label(&self, new_label:&str)->Rc<Self>{
        Self::new(
            self.start_position,
            self.end_position,
            Some(new_label.into()),
            self.children,
        )
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