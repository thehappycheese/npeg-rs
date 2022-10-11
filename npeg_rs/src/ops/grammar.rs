use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
pub struct Grammar {
    id                  : OpaqueIdentifier,
    rule_set:Vec<(Rc<String>, Rc<dyn Parser>)>,
    starting_rule_name:Option<String>
}
impl Grammar {
    pub fn new(starting_rule:Option<&str>, rules:Vec<(&str, Rc<dyn Parser>)>) -> Self{
        // TODO: check for duplicate rule definitions
        // TODO: if the top level rules are Self::Label this is an issue; the grammar parser will override the Label
        Self {
            id: OpaqueIdentifier::new(),
            rule_set:rules.into_iter().map(|(name, rule)|(Rc::new(name.into()), rule)).collect(),
            starting_rule_name:starting_rule.map(|item| item.to_owned())
            
        }
    }
    /// Takes a string and returns the corresponding rule, if it exists.
    /// The result is an Rc::clone() of the original data
    pub fn get_rule_by_name(&self, rule_name:&str) -> Option<(Rc<String>, Rc<dyn Parser>)>{
        self
        .rule_set
        .iter()
        .find(|(each_rule_name, _each_parser_operator)| rule_name==&each_rule_name[..])
        .cloned()
    }
    
    pub fn get_starting_rule(&self) -> Option<(Rc<String>, Rc<dyn Parser>)>{
        self
        .starting_rule_name
        .as_ref()
        .map(|rule_name|self.get_rule_by_name(rule_name))
        .flatten()
        .or(self.rule_set.first().cloned())
    }
}

impl Parser for Grammar {
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        context.push_rule_set(self.clone());
        if let Some((rule_name, parser_operator)) = context.get_starting_rule(){
            let result = parser_operator
                .parse(context, start_position)
                .map(|res| res.with_label(rule_name));
            context.pop_rule_set();
            result
        }else{
            // TODO: we couldn't find a starting rule somehow
            panic!("Starting rule not found")
        }
    }
}