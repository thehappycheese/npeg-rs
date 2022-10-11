use std::rc::Rc;
use std::fmt::Debug;

use super::{
    parser_match::ParserMatch,
    parser_context::ParserContext,
};

pub trait Parser: Debug {
    fn parse(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>>{
        // Try to lookup previously computed value
        match context.get_memory(start_position, self.get_id()){
            Some(result)=> result,
            None=>{
                // if the cache failed, try to do the parse
                let result = self.clone().parse_internal(context, start_position);
                // cache the result
                context.set_memory(self.get_id(), start_position, result.as_ref().cloned());
                // finally, return the result
                result
            }
        }
    }
    fn parse_internal(self:Rc<Self>, context: &mut Box<ParserContext>, start_position: usize) -> Option<Rc<ParserMatch>>;
    fn get_id(&self)->usize;
}