use super::parser_match::ParserMatch;
use super::parser_operator::ParserOperator;

pub struct ParseContext<'ft> {
    full_text: &'ft str,
}

pub trait Parser {
    fn parse<'a>(&self, full_text: &'a str, start_position: usize) -> Option<ParserMatch<'a>>;
}

impl Parser for ParserOperator {
    fn parse<'a>(&self, full_text: &'a str, start_position: usize) -> Option<ParserMatch<'a>> {
        match self {
            ParserOperator::Literal { literal_text } => {
                if full_text[start_position..].starts_with(literal_text) {
                    Some(ParserMatch {
                        start: start_position,
                        end: start_position + literal_text.len(),
                        label: None,
                    })
                } else {
                    None
                }
            }
            ParserOperator::Regex { regular_expression } => {
                let text_to_match = &full_text[start_position..];
                regular_expression.find(text_to_match).map(|re_match| {
                    if re_match.start() != 0 {
                        panic!("Regular expression matched but not at the specified position")
                    }
                    // TODO: verify that we obtain the correct length for the regular expression match
                    ParserMatch {
                        start: start_position,
                        end: start_position + re_match.end() - re_match.start(),
                        label: None,
                    }
                })
            }
            ParserOperator::Sequence { children } => {
                let mut end_position = start_position;
                let sub_matches: Vec<ParserMatch<'a>> = children
                    .iter()
                    .map_while(|child| match child.parse(full_text, end_position) {
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
                        child.parse(full_text, end_position).map(|ma| {
                            end_position += ma.len();
                            ma
                        })
                    })
                    // TODO: Capture submatch
                    .map(|sub_match| ParserMatch {
                        start: start_position,
                        end: end_position,
                        label: None,
                    })
            }

            ParserOperator::Quantity {
                child,
                minimum_occurrences,
                maximum_occurrences,
            } => {
                let mut end_position = start_position;
                let sub_matches: Vec<ParserMatch<'a>> = (0..*maximum_occurrences)
                    .map_while(|_| match child.parse(full_text, end_position) {
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
                    })
                }
            }
            ParserOperator::Lookahead { child, scout, accept_match } => {
                let mut end_position = start_position;
                let res_child = child.parse(full_text, start_position).map(|parser_match| {
                    end_position += parser_match.len();
                    parser_match
                });
                let res_scout = scout.parse(full_text, end_position);
                if (*accept_match && res_scout.is_some()) || (!accept_match && res_scout.is_none()) {
                    res_child
                } else {
                    None
                }
            }
        }
    }
}
