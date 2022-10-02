use std::{rc::Rc, cell::RefCell};

use super::{
    parser_match::ParserMatch,
    parser_context::ParserContext,
    parser_operator::ParserOperator
};

pub trait Parser {
    fn parse(&self, context: RefCell<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>>;
}

impl Parser for ParserOperator {
    fn parse(&self, context:RefCell<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {

        // Try to lookup previously computed value:
        if let Some(result) = context.borrow_mut().get_memory(start_position, self.get_id()){
            return result
        }

        // TODO: search parse context for previous attempt to parse this ParserOperator instance at the same start_position
        let result:Option<Rc<ParserMatch>> = match self {
            ParserOperator::Grammar { parser_rule_set, id} => {
                context.borrow_mut().push_rule_set(parser_rule_set.clone());
                
                if let Some((rule_name, parser_operator)) = context.borrow().get_starting_rule(){
                    let result = parser_operator
                        .parse(context, start_position)
                        .map(|res| res.with_label(rule_name));
                    context.borrow_mut().pop_rule_set();
                    result
                }else{
                    // TODO: we couldn't find a starting rule somehow
                    panic!("Starting rule not found")
                }
            }

            ParserOperator::Label { child, label, id } => {
                child.parse(context, start_position).map(|item| item.with_label(label.clone()))
            }

            ParserOperator::RuleReference { rule_name , id} => {
                if let Some((rule_name, parser_operator)) = context.get_mut().get_rule(rule_name){
                    parser_operator.parse(context, start_position).map(|res| res.with_label(rule_name))
                }else{
                    // TODO: Probably the user would like a nice message, not a crash
                    panic!("Invalid rule reference not found in current grammar")
                }
            }

            ParserOperator::Literal { literal_text , id} => {
                if context.borrow().get_full_text()[start_position..].starts_with(&literal_text[..]) {
                    Some(ParserMatch::new(
                        start_position,
                        start_position + literal_text.len(),
                        None,
                        vec![].into()
                    ))
                } else {
                    None
                }
            }

            ParserOperator::Regex{ pattern, multi_line, case_insensitive, dot_matches_new_line , id} => {
                let regex = context.borrow().get_compiled_regex(pattern, *multi_line, *case_insensitive, *dot_matches_new_line);
                let text_to_match = &context.borrow().get_full_text()[start_position..];
                regex.find(text_to_match).map(|re_match| {
                    if re_match.start() != 0 {
                        panic!("Regular expression matched but not at the specified position")
                    }
                    // TODO: verify that we obtain the correct length for the regular expression match
                    // The regex library talks some nonsense about byte offsets for unicode...
                    ParserMatch::new(
                        start_position,
                        start_position + re_match.end() - re_match.start(),
                        None,
                        vec![].into(),
                    )
                })
            }
            
            ParserOperator::Sequence { children , id} => {
                let mut end_position = start_position;
                let sub_matches: Vec<Rc<ParserMatch>> = children
                    .iter()
                    .map_while(|child|
                        match child.parse(context, end_position) {
                            Some(ma) => {
                                end_position += ma.len();
                                Some(ma.clone())
                            }
                            _ => None,
                        }
                    )
                    .collect();
                if sub_matches.len() == children.len() {
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

            ParserOperator::Alternation { children , id} => {
                let mut end_position = start_position;
                children
                    .iter()
                    .find_map(|child|
                        child.parse(context, end_position).map(|ma| {
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

            ParserOperator::Quantity {
                child,
                minimum_occurrences,
                maximum_occurrences,
                id
            } => {
                let mut end_position = start_position;
                let sub_matches: Vec<Rc<ParserMatch>> = (0..*maximum_occurrences)
                    .map_while(|_| match child.parse(context, end_position) {
                        Some(ma) => {
                            end_position += ma.len();
                            Some(ma.clone())
                        }
                        _ => None,
                    })
                    .collect();
                if sub_matches.len() < *minimum_occurrences || sub_matches.len() > *maximum_occurrences {
                    None
                } else {
                    // TODO: Capture sub matches
                    Some(ParserMatch::new(
                        start_position,
                        end_position,
                        None,
                        sub_matches.into()
                    ))
                }
            }

            ParserOperator::Lookahead { child, scout, accept_match, id} => {
                let mut end_position = start_position;
                let res_child = child.parse(context, start_position).as_ref().map(|parser_match| {
                    end_position += parser_match.len();
                    parser_match.clone()
                });
                let res_scout = scout.parse(context, end_position);
                if (*accept_match && res_scout.is_some()) || (!accept_match && res_scout.is_none()) {
                    res_child
                } else {
                    None
                }
            }
        };
        context.get_mut().set_memory(self.get_id(), start_position, result)
    }
}
