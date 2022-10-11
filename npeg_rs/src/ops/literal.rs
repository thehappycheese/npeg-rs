use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
pub struct Literal {
    id                  : OpaqueIdentifier,
    literal_text        : String,
}
impl Literal{
    pub fn new(literal_text: &str) -> Self {
        if literal_text.len() == 0 {
            panic!("Zero Length Literal is not permitted")
        }
        Self {
            id:OpaqueIdentifier::new(),
            literal_text: literal_text.into(),
        }
    }
}
impl Parser for Literal{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        if context.get_full_text()[start_position..].starts_with(&self.literal_text[..]) {
            Some(ParserMatch::new(
                start_position,
                start_position + self.literal_text.len(),
                None,
                vec![].into()
            ))
        } else {
            None
        }
    }
}