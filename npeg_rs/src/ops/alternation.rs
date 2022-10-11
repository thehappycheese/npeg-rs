use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
// TODO: rename to FirstAlternative

pub struct Alternation {
    id                  : OpaqueIdentifier,
    children            : Vec<Rc<dyn Parser>>,
}
impl Alternation{
    pub fn new(children: Vec<Rc<dyn Parser>>) -> Self {
        if children.len() == 0 {
            panic!("Zero Length Alternations are not permitted")
        }
        Self {
            id:OpaqueIdentifier::new(),
            children:children
        }
    }
}
impl Parser for Alternation{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        let mut end_position = start_position;
        self.children
            .iter()
            .find_map(|child|
                child.clone().parse(context, end_position).map(|ma| {
                    end_position += ma.len();
                    ma
                })
            )
            .as_ref()
            .map(|sub_match|
                ParserMatch::new(
                    start_position,
                    end_position,
                    None,
                    vec![sub_match.clone()].into()
                )
            )
    }
}