use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
/// Quantity (Repetitions)
///  [exp]+ or [exp]* or [exp]? or [exp]{x:y}
pub struct Quantity {
    id                  : OpaqueIdentifier,
    child               : Rc<dyn Parser>,
    minimum_occurrences : usize,
    maximum_occurrences : usize,
}

impl Quantity{
    pub fn new(
        child: Rc<dyn Parser>,
        minimum_occurrences: usize,
        maximum_occurrences: Option<usize>,
    ) -> Self {
        let maximum_occurrences = maximum_occurrences.unwrap_or(usize::max_value());
        if maximum_occurrences - minimum_occurrences < 1 {
            panic!("Zero length Quantity is not permitted")
        }
        Self {
            id:OpaqueIdentifier::new(),
            child: child,
            minimum_occurrences,
            maximum_occurrences,
        }
    }
}

impl Parser for Quantity{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        let mut end_position = start_position;
        let sub_matches: Vec<Rc<ParserMatch>> = (0..self.maximum_occurrences)
            .map_while(|_| match self.child.clone().parse(context, end_position) {
                Some(ma) => {
                    end_position += ma.len();
                    Some(ma.clone())
                }
                _ => None,
            })
            .collect();
        if sub_matches.len() < self.minimum_occurrences || sub_matches.len() > self.maximum_occurrences {
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
}