use std::rc::Rc;

use super::ParserOperator;
/////////////////////////
///   ParserRuleSet   ///
/////////////////////////
#[derive(Debug, PartialEq, Eq)]
pub struct ParserRuleSet{
    pub rule_set:Vec<(Rc<String>, Rc<ParserOperator>)>,
    pub starting_rule_name:Option<String>
}

impl ParserRuleSet{
    /// Takes a string and returns the corresponding rule, if it exists.
    /// The result is an Rc::clone() of the original data
    pub fn get_rule_by_name(&self, rule_name:&str) -> Option<(Rc<String>, Rc<ParserOperator>)>{
        self
        .rule_set
        .iter()
        .find(|(each_rule_name, _each_parser_operator)| rule_name==&each_rule_name[..])
        .cloned()
    }
    /// test
    pub fn get_starting_rule(&self) -> Option<(Rc<String>, Rc<ParserOperator>)>{
        self
        .starting_rule_name
        .as_ref()
        .map(|rule_name|self.get_rule_by_name(rule_name))
        .flatten()
        .or(self.rule_set.first().cloned())
    }
}