use std::rc::Rc;

use super::{
    parser_rule_set::ParserRuleSet,
    opaque_identifier::OpaqueIdentifier,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserOperator {
    Grammar {
        id                  : OpaqueIdentifier,
        parser_rule_set     : Rc<ParserRuleSet>,
    },
    // label:[exp]
    Label {
        id                  : OpaqueIdentifier,
        child               : Rc<ParserOperator>,
        label               : Rc<String>,
    },
    // rule
    RuleReference {
        id                  : OpaqueIdentifier,
        rule_name           : Rc<String>,
    },
    // "..."
    Literal {
        id                  : OpaqueIdentifier,
        literal_text        : Rc<String>,
    },
    // re"..."is
    Regex {
        id                  : OpaqueIdentifier,
        pattern             : Rc<String>,
        multi_line          : bool,
        case_insensitive    : bool,
        dot_matches_new_line: bool
    },
    // [exp] [exp] [exp]
    Sequence {
        id                  : OpaqueIdentifier,
        children            : Vec<Rc<ParserOperator>>,
    },
    // [exp] / [exp] / [exp]
    Alternation {
        id                  : OpaqueIdentifier,
        children            : Vec<Rc<ParserOperator>>,
    },
    // [exp]+ or [exp]* or [exp]?
    Quantity {
        id                  : OpaqueIdentifier,
        child               : Rc<ParserOperator>,
        minimum_occurrences : usize,
        maximum_occurrences : usize,
    },
    // ![exp] or &[exp]
    Lookahead {
        id                  : OpaqueIdentifier,
        child               : Rc<ParserOperator>,
        scout               : Rc<ParserOperator>,
        accept_match        : bool,
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

    pub fn grammar(starting_rule:Option<&str>, rules:Vec<(&str, ParserOperator)>) -> Self{
        // TODO: check for duplicate rule definitions
        // TODO: if the top level rules are Self::Label this is an issue; the grammar parser will override the Label
        Self::Grammar {
            id: OpaqueIdentifier::new(),
            parser_rule_set: ParserRuleSet {
                rule_set:rules.into_iter().map(|(name, rule)|(Rc::new(name.into()), Rc::new(rule))).collect(),
                starting_rule_name:starting_rule.map(|item| item.to_owned())
            }.into()
        }

    }

    pub fn literal(literal_text: &str) -> ParserOperator {
        if literal_text.len() == 0 {
            panic!("Zero Length Literal is not permitted")
        }
        Self::Literal {
            id:OpaqueIdentifier::new(),
            literal_text: Rc::new(literal_text.to_owned()),
        }
    }
    pub fn regex(pattern: &str, multi_line:bool, case_insensitive:bool,dot_matches_new_line:bool) -> ParserOperator {
        ParserOperator::Regex {
            id:OpaqueIdentifier::new(),
            pattern:if !pattern.starts_with("^") {
                Rc::new("^".to_owned() + pattern)
            } else {
                Rc::new(pattern.to_owned())
            },
            multi_line,
            case_insensitive,
            dot_matches_new_line,
        }
    }
    pub fn sequence(children: Vec<ParserOperator>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero length Sequence is not permitted")
        }
        Self::Sequence { 
            id:OpaqueIdentifier::new(),
            children:children.into_iter().map(|item|item.into()).collect()
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

    pub fn alternation(children: Vec<ParserOperator>) -> ParserOperator {
        if children.len() == 0 {
            panic!("Zero Length Alternations are not permitted")
        }
        Self::Alternation {
            id:OpaqueIdentifier::new(),
            children:children.into_iter().map(|item|item.into()).collect()
        }
    }

    pub fn lookahead(
        child: Self,
        scout: Self,
        accept_match: bool,
    ) -> Self {
        Self::Lookahead {
            id: OpaqueIdentifier::new(),
            child: child.into(),
            scout: scout.into(),
            accept_match,
        }
    }

    pub fn label(child: Self, arg: &str) -> Self {
        // TODO: dont permit label to be under label?
        Self::Label {
            id: OpaqueIdentifier::new(),
            child: child.into(),
            label: Rc::new(arg.into())
        }
    }
    pub fn rule_reference(rule_name:&str)-> Self{
        Self::RuleReference {
            id: OpaqueIdentifier::new(),
            rule_name: Rc::new(rule_name.into())
        }
    }
}