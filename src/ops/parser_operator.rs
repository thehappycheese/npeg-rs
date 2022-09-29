
use regex::Regex;


pub struct Grammar{

}
impl Grammar{}


#[derive(Clone)]
pub enum ParserOperator {
    // "..."
    Literal {
        literal_text: String,
    },
    // re"..."is
    Regex {
        regular_expression: Regex,
    },
    // [exp] [exp] [exp]
    Sequence {
        children: Vec<ParserOperator>,
    },
    // [exp] / [exp] / [exp]
    Alternation {
        children: Vec<ParserOperator>,
    },
    // [exp]+ or [exp]* or [exp]?
    Quantity {
        child: Box<ParserOperator>,
        minimum_occurrences: usize,
        maximum_occurrences: usize,
    },
    // ![exp] or &[exp]
    Lookahead {
        child: Box<ParserOperator>,
        scout: Box<ParserOperator>,
        accept_match: bool,
    },
}


impl ParserOperator {
    pub fn literal(literal_text: &str) -> ParserOperator {
        if literal_text.len() == 0 {
            panic!("Zero Length Literal is not permitted")
        }
        Self::Literal {
            literal_text: literal_text.into(),
        }
    }
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