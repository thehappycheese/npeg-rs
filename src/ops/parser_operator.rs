
use regex::Regex;
use super::parser::Parser;
use super::parser_match::ParserMatch;

#[derive(Clone, Debug)]
pub struct Grammar{
    rules:Vec<Box<dyn Parser>>
}
impl Parser for Grammar{
    fn parse<'a>(&self, full_text: &'a str, start_position: usize) -> Option<ParserMatch<'a>> {
        // TODO: implement grammar
        todo!("Not done yet")
    }
}

// "..."
#[derive(Clone, Debug)]
struct LiteralNode {
    literal_text: String,
    label_text:Option<String>
}
impl LiteralNode{
    fn new(literal_text: &str) -> LiteralNode {
        if literal_text.len() == 0 {
            panic!("Zero Length Literal is not permitted")
        }
        Self::Literal {
            literal_text: literal_text.into(),
        }
    }
}
impl Parser for LiteralNode{
    fn parse<'a>(&self, full_text: &'a str, start_position: usize) -> Option<ParserMatch<'a>> {
        if full_text[start_position..].starts_with(self.literal_text) {
            Some(ParserMatch {
                start: start_position,
                end: start_position + self.literal_text.len(),
                label: None,
            })
        } else {
            None
        }
    }
    fn label(&self)->Option<&str> {
        return self.label_text.into()
    }
}


// re"..."is
#[derive(Clone, Debug)]
struct RegexNode {
    regular_expression: Regex,
}

// [exp] [exp] [exp]
#[derive(Clone, Debug)]
struct SequenceNoce {
    children: Vec<Box<dyn Parser>>,
}

// [exp] / [exp] / [exp]
#[derive(Clone, Debug)]
struct Alternation {
    children: Vec<Box<dyn Parser>>,
}

// [exp]+ or [exp]* or [exp]?
#[derive(Clone, Debug)]
struct Quantity {
    child: Box<dyn Parser>,
    minimum_occurrences: usize,
    maximum_occurrences: usize,
}

// ![exp] or &[exp]
#[derive(Clone, Debug)]
struct LookaheadPositive {
    child: Box<dyn Parser>,
    scout: Box<dyn Parser>,
}

#[derive(Clone, Debug)]
struct LookaheadNegative {
    child: Box<dyn Parser>,
    scout: Box<dyn Parser>,
}


impl ParserOperator {
    pub fn regex(regular_expression: &str) -> ParserOperator {
        let regular_expression: String = if !regular_expression.starts_with("^") {
            "^".to_owned() + regular_expression
        } else {
            regular_expression.into()
        };
        match Regex::new(regular_expression.as_str()) {
            Ok(regular_expression) => ParserOperator::Regex { regular_expression },
            Err(e) => {
                println!("{:?}", e);
                panic!("Failed to parse regular expression");
            }
        }
    }
    pub fn sequence(children: Vec<ParserOperator>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero length Sequence is not permitted")
        }
        Self::Sequence { children }
    }
    pub fn quantity(
        child: ParserOperator,
        minimum_occurrences: usize,
        maximum_occurrences: Option<usize>,
    ) -> ParserOperator {
        let maximum_occurrences = maximum_occurrences.unwrap_or(usize::max_value());
        if maximum_occurrences - minimum_occurrences < 1 {
            panic!("Zero length Quantity is not permitted")
        }
        Self::Quantity {
            child: child.into(),
            minimum_occurrences,
            maximum_occurrences,
        }
    }

    pub fn alternation(children: Vec<ParserOperator>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero Length Alternations are not permitted")
        }
        Self::Alternation { children }
    }

    pub fn lookahead(
        child: ParserOperator,
        scout: ParserOperator,
        accept_match: bool,
    ) -> ParserOperator {
        Self::Lookahead {
            child: child.into(),
            scout: scout.into(),
            accept_match,
        }
    }
}