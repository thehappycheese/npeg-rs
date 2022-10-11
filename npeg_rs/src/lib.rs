
mod core;
mod ops;

pub use crate::ops::{
    Alternation,
    Grammar,
    Label,
    Literal,
    Lookahead,
    Quantity,
    Regex,
    RuleReference,
    Sequence,
};

pub use crate::core::{
    Parser,
    ParserContext
};

#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests{
    use std::rc::Rc;
    use super::*;
    #[test]
    fn test_grammar() {

        let gram = Rc::new(
            Grammar::new(
                None,
                vec![
                    ("Prog",  seq!(lit!("("), rul!("Quant"))),
                    ("Quant", qtt!(rul!("Wurd"), 0, Some(5))),
                    ("Wurd",  seq!(rul!("Atom"), rul!("Btom"))),
                    ("Atom",  lit!("a")),
                    ("Btom",  lit!("b")),
                ]
            )
        );

        let test_string = "( ";

        let mut context = Box::new(ParserContext::new(test_string));
        println!("{:?}", gram.parse(&mut context, 0));

        //let re = Regex::new("^[ab]").unwrap();
        //println!("{:?}", re.find(&"tac"[1..]));
    }

    #[test]
    fn test_a(){
        let lit_a = lit!("a");
        let lit_b = lit!("a()");
        //let qtt = Quantity::new(lit_a.clone(), 1, Some(3));
        //let lah = Lookahead::new(qtt.clone(), ParserOperator::literal("("), true);
        //let reg = Rc::new(Regex::new("^[0-9]", false, false, false));
        let reg = reg!("[0-9]" i);
        //let seq = Sequence::new(vec![qtt.clone(), reg, lit_b.clone()]);
        let lab = lbl!(lit_a, "ATOM A");
        let alt = alt!(lit_b, lab, reg);
        let mut context = Box::new(ParserContext::new("0"));
        println!("{:?}", alt.parse(&mut context, 0))
    }
}
