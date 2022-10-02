use regex::{Regex, RegexBuilder};

use std::{collections::BTreeMap, rc::Rc};

use super::{parser_match::ParserMatch, parser_operator::ParserOperator, parser_rule_set::ParserRuleSet};

pub struct ParserContext<'ft> {
    full_text: &'ft str,
    memory: BTreeMap<(usize, usize), Option<Rc<ParserMatch>>>,
    rule_sets: Vec<Rc<ParserRuleSet>>,
}

impl<'ft> ParserContext<'ft> {
    pub fn new(full_text: &'ft str) -> ParserContext {
        ParserContext {
            full_text,
            memory: BTreeMap::new(),
            rule_sets: vec![],
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
        // Cant do this:
        //&parser_match

        // TODO: we have to do an unwrap here to borrow the value we stored in memory... sucks to have to look it up again :/
        parser_match
    }

    pub fn push_rule_set(&mut self, rule_set: Rc<ParserRuleSet>) {
        self.rule_sets.push(rule_set)
    }
    pub fn pop_rule_set(&mut self) {
        self.rule_sets.pop();
    }
    pub fn get_rule(&self, rule_name: &str) -> Option<(Rc<String>, Rc<ParserOperator>)> {
        self.rule_sets
        .last()
        .map(|rule_set| rule_set.get_rule_by_name(rule_name))
        .flatten()
    }
    pub fn get_starting_rule(&self) -> Option<(Rc<String>, Rc<ParserOperator>)> {
        self.rule_sets
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
