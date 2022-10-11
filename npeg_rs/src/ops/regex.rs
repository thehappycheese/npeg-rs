use std::rc::Rc;
use crate::core::{
    OpaqueIdentifier,
    Parser,
    ParserContext,
    ParserMatch,
};
#[derive(Debug)]
pub struct Regex {
    id                   : OpaqueIdentifier,
    pattern              : String,
    multi_line           : bool,
    case_insensitive     : bool,
    dot_matches_new_line : bool
}
impl Regex{
    pub fn new(pattern: &str, multi_line:bool, case_insensitive:bool,dot_matches_new_line:bool) -> Self {
        Self {
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
}
impl Parser for Regex{
    fn get_id(&self)->usize {
        self.id.id()
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>> {
        let regex = context.get_compiled_regex(
            &self.pattern[..],
            self.multi_line,
            self.case_insensitive,
            self.dot_matches_new_line
        );
        let text_to_match = &context.get_full_text()[start_position..];
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
}