use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
pub struct Lookahead {
    id                  : OpaqueIdentifier,
    child               : Rc<dyn Parser>,
    scout               : Rc<dyn Parser>,
    accept_match        : bool,
}
impl Lookahead{
    pub fn new(
        child: Rc<dyn Parser>,
        scout: Rc<dyn Parser>,
        accept_match: bool,
    ) -> Self {
        Self {
            id: OpaqueIdentifier::new(),
            child: child.into(),
            scout: scout.into(),
            accept_match,
        }
    }
}

impl Parser for Lookahead{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        let mut end_position = start_position;
        let res_child = self.child.clone().parse(context, start_position).as_ref().map(|parser_match| {
            end_position += parser_match.len();
            parser_match.clone()
        });
        let res_scout = self.scout.clone().parse(context, end_position);
        if (self.accept_match && res_scout.is_some()) || (!self.accept_match && res_scout.is_none()) {
            res_child
        } else {
            None
        }
    }
}