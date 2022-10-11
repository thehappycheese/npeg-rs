
use std::rc::Rc;



/// `ParserMatch`
/// 
/// The result of a successful `ParserOperator::parse(...)`
/// 
/// `ParserMatch` may or may not have a label which is assigned to the match as part of the `parse()` process
/// 
/// The label
/// 
#[derive(Debug)]
pub struct ParserMatch {
    start_position: usize,
    end_position: usize,
    label: Option<Rc<String>>,
    children: Rc<Vec<Rc<ParserMatch>>>,
}
impl ParserMatch {
    pub fn new(start_position: usize, end_position: usize, label: Option<Rc<String>>, children: Rc<Vec<Rc<Self>>>) -> Rc<Self>{
        // Only allow obtain reference behind Rc
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
    pub fn with_label(&self, new_label:Rc<String>)->Rc<Self>{
        Self::new(
            self.start_position,
            self.end_position,
            Some(new_label),
            self.children.clone(),
        )
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::ParserMatch;
    #[test]
    fn match_gets_correct_substring() {
        let m = ParserMatch::new(
            0,
            1,
            None,
            Rc::new(vec![]),
        );
        let full_text = "0123456789";
        assert_eq!(m.get_text(full_text), "0");
    }

    #[test]
    fn match_gets_correct_substring_unicode() {
        let m = ParserMatch::new(
            0,
            "0✔️".len(),
            None,
            Rc::new(vec![]),
        );
        let full_text = "0✔️23456789";
        assert_eq!(m.get_text(full_text), "0✔️");
    }
}
