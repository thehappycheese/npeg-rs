use std::collections::HashMap;

use regex::RegexBuilder;
use regex::Regex;

use super::parser_match::ParserMatch;
use super::parser_operator;
use super::parser_operator::{ParserOperator, ParserRuleSet};




pub struct ParserContext<'ft> {
    full_text: &'ft str,
    memory:HashMap<(&'ft ParserOperator, usize), Option<ParserMatch>>,
    rule_sets: Vec<&'ft ParserRuleSet>
}

impl<'ft> ParserContext<'ft>{
    fn get_full_text(&self)->&str {
        self.full_text
    }
    fn get_memory(&self, parser_operator:&'ft ParserOperator, start_position:usize)->Option<&Option<ParserMatch>>{
        self.memory.get(&(parser_operator, start_position))
    }
    fn set_memory(&self, parser_operator:&'ft ParserOperator, start_position:usize, parser_match:Option<ParserMatch>)->&Option<ParserMatch>{
        // TODO: when .insert succeeds the key is not updated
        //       this can be a problem for types that can be == without being the same instance.
        //       (our grammar may include several == subtrees which are NOT the same instances
        //        ideally we would de-duplicate these subtrees before calling parse... but lets say it's not guaranteed for now...)
        //       `key(parser_operator instance1,..) == key(parser_operator instance2,..)` is the desired behavior, HOWEVER
        //       it is preferable to check `key(parser_operator1,..) is key(parser_operator2,..)`,
        //       rather than doing all the work for a deeply nested == comparison
        
        //       OK so it turns out this is only achievable using unsafe raw pointers. Soooo that sucks.
        if let Some(_old_value) = self.memory.insert((parser_operator, start_position), parser_match){
            // TODO: If we try re-insert over the same key, this is not the user's fault. Try to add test case or something?
            panic!("Reinserted over same memory")
        };
        &parser_match
        // TODO: every time the parser steps forward, we can probably clear parts of this map where start_position < new_position
    }

    fn push_rule_set(&self, rule_set:&'ft ParserRuleSet){
        self.rule_sets.push(rule_set)
    }
    fn pop_rule_set(&self){
        self.rule_sets.pop();
    }
    fn get_rule(&self, rule_name:&str)->Option<&(String, ParserOperator)>{
        self.rule_sets.last().map(|rule_set| rule_set.get_rule_by_name(rule_name)).flatten()
    }
    fn get_starting_rule(&self)->Option<&(String, ParserOperator)>{
        self.rule_sets.last().map(|rule_set|rule_set.get_starting_rule()).flatten()
    }
    fn get_compiled_regex(&self, pattern:&str, multi_line:bool, case_insensitive:bool, dot_matches_new_line:bool) -> Regex{
        // TODO:  memoize; this is super slow to do repeatedly
        match RegexBuilder::new(pattern).multi_line(multi_line).case_insensitive(case_insensitive).dot_matches_new_line(dot_matches_new_line).build(){
            Ok(regex)=>regex,
            Err(err)=>{
                panic!("TODO: Regex did not parse, this is a user error I think?")
            }
        }
    }

}

pub trait Parser {
    fn parse<'a>(&self, context: &'a mut ParserContext, start_position: usize) -> &Option<ParserMatch>;
}

impl Parser for ParserOperator {
    fn parse<'a>(&self, context: &'a mut ParserContext, start_position: usize) -> &Option<ParserMatch> {
        if let Some(result) = context.get_memory(self, start_position){
            return result
        }
        // TODO: search parse context for previous attempt to parse this ParserOperator instance at the same start_position
        let result:Option<ParserMatch> = match self {
            ParserOperator::Grammar (parser_rule_set) => {//ParserRuleSet{rule_set, starting_rule})=>{
                context.push_rule_set(parser_rule_set);
                
                if let Some((rule_name, parser_operator)) = context.get_starting_rule(){
                    let result = parser_operator.parse(context, start_position).map(|res|res.with_label(rule_name));
                    context.pop_rule_set();
                    result
                }else{
                    // TODO: we couldn't find a starting rule somehow
                    panic!("Starting rule not found")
                }
            }

            ParserOperator::Label { child, label } => {
                child.parse(context, start_position).map(|item| item.with_label(label))
            }

            ParserOperator::RuleReference { rule_name } => {
                if let Some((rule_name, parser_operator)) = context.get_rule(rule_name){
                    parser_operator.parse(context, start_position).map(|res|res.with_label(rule_name))
                }else{
                    // TODO: Probably the user would like a nice message, not a crash
                    panic!("Invalid rule reference not found in current grammar")
                }
            }

            ParserOperator::Literal { literal_text } => {
                if context.full_text[start_position..].starts_with(literal_text) {
                    Some(ParserMatch {
                        start: start_position,
                        end: start_position + literal_text.len(),
                        label: None,
                        children: vec![]
                    })
                } else {
                    None
                }
            }

            ParserOperator::Regex{ pattern, multi_line, case_insensitive, dot_matches_new_line } => {
                let regex = context.get_compiled_regex(pattern, *multi_line, *case_insensitive, *dot_matches_new_line);
                let text_to_match = &context.full_text[start_position..];
                regex.find(text_to_match).map(|re_match| {
                    if re_match.start() != 0 {
                        panic!("Regular expression matched but not at the specified position")
                    }
                    // TODO: verify that we obtain the correct length for the regular expression match
                    // The regex library talks some nonsense about byte offsets for unicode...
                    ParserMatch {
                        start: start_position,
                        end: start_position + re_match.end() - re_match.start(),
                        label: None,
                        children: vec![],
                    }
                })
            }
            
            ParserOperator::Sequence { children } => {
                let mut end_position = start_position;
                let sub_matches: Vec<&ParserMatch> = children
                    .iter()
                    .map_while(|child| match child.parse(context, end_position) {
                        Some(ma) => {
                            end_position += ma.len();
                            Some(ma)
                        }
                        _ => None,
                    })
                    .collect();
                if sub_matches.len() == children.len() {
                    // TODO: Capture sub matches
                    Some(ParserMatch {
                        start: start_position,
                        end: end_position,
                        label: None,
                        children: sub_matches
                    })
                } else {
                    None
                }
            }

            ParserOperator::Alternation { children } => {
                let mut end_position = start_position;
                children
                    .iter()
                    .find_map(|child| {
                        child.parse(context, end_position).map(|ma| {
                            end_position += ma.len();
                            ma
                        })
                    })
                    .map(|sub_match| ParserMatch {
                        start: start_position,
                        end: end_position,
                        label: None,
                        children: vec![sub_match]
                    })
            }

            ParserOperator::Quantity {
                child,
                minimum_occurrences,
                maximum_occurrences,
            } => {
                let mut end_position = start_position;
                let sub_matches: Vec<&ParserMatch> = (0..*maximum_occurrences)
                    .map_while(|_| match child.parse(context, end_position) {
                        Some(ma) => {
                            end_position += ma.len();
                            Some(ma)
                        }
                        _ => None,
                    })
                    .collect();
                if sub_matches.len() < *minimum_occurrences || sub_matches.len() > *maximum_occurrences {
                    None
                } else {
                    // TODO: Capture sub matches
                    Some(ParserMatch {
                        start: start_position,
                        end: end_position,
                        label: None,
                        children: sub_matches
                    })
                }
            }

            ParserOperator::Lookahead { child, scout, accept_match } => {
                let mut end_position = start_position;
                let res_child = child.parse(context, start_position).map(|parser_match| {
                    end_position += parser_match.len();
                    parser_match
                });
                let res_scout = scout.parse(context, end_position);
                if (*accept_match && res_scout.is_some()) || (!accept_match && res_scout.is_none()) {
                    res_child
                } else {
                    None
                }
            }
        };
        context.set_memory(self, start_position, result)
    }
}
