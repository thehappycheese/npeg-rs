
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParserRuleSet{
    rule_set:Vec<(String, ParserOperator)>,
    starting_rule:Option<String>
}

impl ParserRuleSet{
    pub fn get_rule_by_name(&self, rule_name:&str) -> Option<&(String, ParserOperator)>{
        self.rule_set
        .iter()
        .find(|(each_rule_name, each_parser_operator)| rule_name==each_rule_name)
    }
    pub fn get_starting_rule(&self) -> Option<&(String, ParserOperator)>{
        self.starting_rule.as_ref().map(|rule_name|self.get_rule_by_name(rule_name)).flatten().or(self.rule_set.first())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParserOperator {

    Grammar (ParserRuleSet),

    Label {
        child:Box<ParserOperator>,
        label:String
    },

    // rule
    RuleReference {
        rule_name:String
    },

    // "..."
    Literal {
        literal_text: String,
    },
    // re"..."is
    Regex {
        pattern: String,
        multi_line: bool,
        case_insensitive: bool,
        dot_matches_new_line: bool
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
    }
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
    pub fn regex(pattern: &str, multi_line:bool, case_insensitive:bool,dot_matches_new_line:bool) -> ParserOperator {
        ParserOperator::Regex {
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