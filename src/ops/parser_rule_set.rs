use super::ParserOperator;
/////////////////////////
///   ParserRuleSet   ///
/////////////////////////
#[derive(Debug, PartialEq, Eq)]
pub struct ParserRuleSet{
    pub rule_set:Vec<(String, ParserOperator)>,
    pub starting_rule:Option<String>
}

impl ParserRuleSet{
    pub fn get_rule_by_name(&self, rule_name:&str) -> Option<&(String, ParserOperator)>{
        self.rule_set
        .iter()
        .find(|(each_rule_name, _each_parser_operator)| rule_name==each_rule_name)
    }
    pub fn get_starting_rule(&self) -> Option<&(String, ParserOperator)>{
        self.starting_rule.as_ref().map(|rule_name|self.get_rule_by_name(rule_name)).flatten().or(self.rule_set.first())
    }
}