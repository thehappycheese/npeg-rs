use regex::{Regex, RegexBuilder};

use std::{collections::BTreeMap, rc::Rc};

use super::{
    ParserMatch,
    Parser,
};
use crate::ops::Grammar;


pub struct ParserContext<'ft> {
    full_text: &'ft str,
    memory: BTreeMap<(usize, usize), Option<Rc<ParserMatch>>>,
    current_grammar: Vec<Rc<Grammar>>,
}

impl<'ft> ParserContext<'ft> {
    pub fn new(full_text: &'ft str) -> ParserContext {
        ParserContext {
            full_text,
            memory: BTreeMap::new(),
            current_grammar: vec![],
        }
    }
    pub fn get_full_text(&self) -> &str {
        self.full_text
    }
    pub fn get_memory(&self, start_position: usize, parser_operator_id: usize) -> Option<Option<Rc<ParserMatch>>> {
        self.memory.get(&(start_position, parser_operator_id)).map(|item|item.clone())
    }
    pub fn set_memory(& mut self, start_position: usize, parser_operator_id: usize, parser_match: Option<Rc<ParserMatch>>) -> Option<Rc<ParserMatch>> {
        // TODO: every time the parser steps forward, we can abandon parts of this map where start_position < new_position
        if let Some(_old_value) = self.memory.insert((start_position, parser_operator_id), parser_match.clone()) {
            // TODO: If we try re-insert over the same key, this is not the user's fault. Try to add test case or something?
            panic!("Reinserted over same key at position {}", start_position)
        };
        parser_match
    }

    pub fn push_rule_set(&mut self, rule_set: Rc<Grammar>) {
        self.current_grammar.push(rule_set)
    }
    pub fn pop_rule_set(&mut self) {
        self.current_grammar.pop();
    }
    pub fn get_rule(&self, rule_name: &str) -> Option<(Rc<String>, Rc<dyn Parser>)> {
        self.current_grammar
        .last()
        .map(|rule_set| rule_set.get_rule_by_name(rule_name))
        .flatten()
    }
    pub fn get_starting_rule(&self) -> Option<(Rc<String>, Rc<dyn Parser>)> {
        self.current_grammar
        .last()
        .map(|rule_set| rule_set.get_starting_rule())
        .flatten()
    }
    pub fn get_compiled_regex(&self, pattern: &str, multi_line: bool, case_insensitive: bool, dot_matches_new_line: bool) -> Regex {
        // TODO:  memoize; this is super slow to do repeatedly
        match RegexBuilder::new(pattern)
            .multi_line(multi_line)
            .case_insensitive(case_insensitive)
            .dot_matches_new_line(dot_matches_new_line)
            .build()
        {
            Ok(regex) => regex,
            Err(_) => {
                panic!("TODO: Regex did not parse {}, this is a user error I think?", pattern)
            }
        }
    }
}
