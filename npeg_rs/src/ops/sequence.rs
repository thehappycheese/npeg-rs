use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
pub struct Sequence {
    id                  : OpaqueIdentifier,
    children            : Vec<Rc<dyn Parser>>,
}
impl Sequence{
    pub fn new(children: Vec<Rc<dyn Parser>>) -> Self {
        if children.len() == 0 {
            panic!("Zero length Sequence is not permitted")
        }
        Self { 
            id:OpaqueIdentifier::new(),
            children:children
        }
    }
}

impl Parser for Sequence{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        let mut end_position = start_position;
        let sub_matches: Vec<Rc<ParserMatch>> = self.children
            .iter()
            .map_while(|child|
                match child.clone().parse(context, end_position) {
                    Some(ma) => {
                        end_position += ma.len();
                        Some(ma.clone())
                    }
                    _ => None,
                }
            )
            .collect();
        if sub_matches.len() == self.children.len() {
            // TODO: Capture sub matches
            Some(ParserMatch::new(
                start_position,
                end_position,
                None,
                sub_matches.into()
            ))
        } else {
            None
        }
    }
}