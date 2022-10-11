use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
// rule
pub struct RuleReference {
    id                  : OpaqueIdentifier,
    rule_name           : String,
}

impl RuleReference{
    pub fn new(rule_name:&str)-> Self{
        Self {
            id: OpaqueIdentifier::new(),
            rule_name: rule_name.into()
        }
    }
}
impl Parser for RuleReference{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        if let Some((_old_rule_name, parser_operator)) = context.get_rule(self.rule_name.as_ref()){
            parser_operator.parse(context, start_position).map(|res| res.with_label(Rc::new(self.rule_name.clone())))
        }else{
            // TODO: Probably the user would like a nice message, not a crash
            panic!("Invalid rule reference not found in current grammar")
        }
    }
}