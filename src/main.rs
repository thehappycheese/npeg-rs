mod ops;
use std::rc::Rc;

use ops::ParserOperator;
use ops::Parser;

use crate::ops::ParserContext;






fn main() {
    let test_string = "aaa(";
    let lit_a = ParserOperator::literal("a");
    let lit_b = ParserOperator::literal("a()");
    //let qtt = ParserOperator::quantity(lit_a.clone(), 1, Some(3));
    //let lah = ParserOperator::lookahead(qtt.clone(), ParserOperator::literal("("), true);
    let reg = ParserOperator::regex("^[0-9]", false, false, false);
    //let seq = ParserOperator::sequence(vec![qtt.clone(), reg, lit_b.clone()]);
    let alt = ParserOperator::alternation(vec![lit_a.into(), lit_b.into(), reg.into()]);
    let context = Rc::new(ParserContext::new(test_string));
    println!("{:?}", alt.parse(context, 0));

    //let re = Regex::new("^[ab]").unwrap();
    //println!("{:?}", re.find(&"tac"[1..]));
}
