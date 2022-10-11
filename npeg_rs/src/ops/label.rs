use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
/// label:[exp]
pub struct Label {
    id                  : OpaqueIdentifier,
    child               : Rc<dyn Parser>,
    label               : Rc<String>,
}

impl Label{
    pub fn new(child: Rc<dyn Parser>, label: &str) -> Self {
        Self{
            id: OpaqueIdentifier::new(),
            child: child,
            label: Rc::new(label.into())
        }
    }
}
impl Parser for Label{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        self.child.clone().parse(context, start_position).map(|item| item.with_label(self.label.clone()))
    }
}