use std::rc::Rc;

use super::{
    parser_rule_set::ParserRuleSet,
    opaque_identifier::OpaqueIdentifier,
};

//////////////////////////
///   ParserOperator   ///
//////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub enum ParserOperator {

    Grammar {
        id:OpaqueIdentifier,
        parser_rule_set:ParserRuleSet,
    },

    // label:[exp]
    Label {
        id:OpaqueIdentifier,
        child:Rc<ParserOperator>,
        label:String
    },

    // rule
    RuleReference {
        id:OpaqueIdentifier,
        rule_name:String
    },

    // "..."
    Literal {
        id:OpaqueIdentifier,
        literal_text: String,
    },
    // re"..."is
    Regex {
        id:OpaqueIdentifier,
        pattern: String,
        multi_line: bool,
        case_insensitive: bool,
        dot_matches_new_line: bool
    },
    // [exp] [exp] [exp]
    Sequence {
        id:OpaqueIdentifier,
        children: Vec<Rc<ParserOperator>>,
    },
    // [exp] / [exp] / [exp]
    Alternation {
        id:OpaqueIdentifier,
        children: Vec<Rc<ParserOperator>>,
    },
    // [exp]+ or [exp]* or [exp]?
    Quantity {
        id:OpaqueIdentifier,
        child: Rc<ParserOperator>,
        minimum_occurrences: usize,
        maximum_occurrences: usize,
    },
    // ![exp] or &[exp]
    Lookahead {
        id:OpaqueIdentifier,
        child: Rc<ParserOperator>,
        scout: Rc<ParserOperator>,
        accept_match: bool,
    }
}


impl ParserOperator {

    pub fn get_id(&self) -> usize {
        match self{
            ParserOperator::Grammar       { id, ..}=>id.id(),
            ParserOperator::Literal       { id, ..}=>id.id(),
            ParserOperator::RuleReference { id, ..}=>id.id(),
            ParserOperator::Regex         { id, ..}=>id.id(),
            ParserOperator::Alternation   { id, ..}=>id.id(),
            ParserOperator::Lookahead     { id, ..}=>id.id(),
            ParserOperator::Quantity      { id, ..}=>id.id(),
            ParserOperator::Sequence      { id, ..}=>id.id(),
            ParserOperator::Label         { id, ..}=>id.id(),
        }
    }

    pub fn literal(literal_text: &str) -> ParserOperator {
        if literal_text.len() == 0 {
            panic!("Zero Length Literal is not permitted")
        }
        Self::Literal {
            id:OpaqueIdentifier::new(),
            literal_text: literal_text.into(),
        }
    }
    pub fn regex(pattern: &str, multi_line:bool, case_insensitive:bool,dot_matches_new_line:bool) -> ParserOperator {
        ParserOperator::Regex {
            id:OpaqueIdentifier::new(),
            pattern:if !pattern.starts_with("^") {
                "^".to_owned() + pattern
            } else {
                pattern.into()
            },
            multi_line,
            case_insensitive,
            dot_matches_new_line,
        }
    }
    pub fn sequence(children: Vec<Rc<ParserOperator>>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero length Sequence is not permitted")
        }
        Self::Sequence { 
            id:OpaqueIdentifier::new(),
            children 
        }
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
            id:OpaqueIdentifier::new(),
            child: child.into(),
            minimum_occurrences,
            maximum_occurrences,
        }
    }

    pub fn alternation(children: Vec<Rc<ParserOperator>>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero Length Alternations are not permitted")
        }
        Self::Alternation {
            id:OpaqueIdentifier::new(),
            children
        }
    }

    pub fn lookahead(
        child: ParserOperator,
        scout: ParserOperator,
        accept_match: bool,
    ) -> ParserOperator {
        Self::Lookahead {
            id:OpaqueIdentifier::new(),
            child: child.into(),
            scout: scout.into(),
            accept_match,
        }
    }
}