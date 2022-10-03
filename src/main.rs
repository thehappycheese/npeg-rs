mod ops;
use ops::{
    ParserOperator,
    Parser,
    ParserContext
};

fn main() {

    let gram = ParserOperator::grammar(
        None,
        vec![
            ("Prog", ParserOperator::sequence(vec![ParserOperator::literal("( "), ParserOperator::rule_reference("Quant")])),
            ("Quant", ParserOperator::quantity(ParserOperator::rule_reference("Wurd"), 0, Some(5))),
            ("Wurd", ParserOperator::sequence(vec![ParserOperator::rule_reference("Atom"), ParserOperator::rule_reference("Btom")])),
            ("Atom", ParserOperator::literal("a")),
            ("Btom", ParserOperator::literal("b")),
    ]);

    let test_string = "( ";
    let lit_a = ParserOperator::literal("a");
    let lit_b = ParserOperator::literal("a()");
    //let qtt = ParserOperator::quantity(lit_a.clone(), 1, Some(3));
    //let lah = ParserOperator::lookahead(qtt.clone(), ParserOperator::literal("("), true);
    let reg = ParserOperator::regex("^[0-9]", false, false, false);
    //let seq = ParserOperator::sequence(vec![qtt.clone(), reg, lit_b.clone()]);
    let lab = ParserOperator::label(lit_a, "ATOM A");
    let alt = ParserOperator::alternation(vec![lit_b.into(), lab.into(), reg.into()]);
    let mut context = Box::new(ParserContext::new(test_string));
    println!("{:?}", gram.parse(&mut context, 0));

    //let re = Regex::new("^[ab]").unwrap();
    //println!("{:?}", re.find(&"tac"[1..]));
}
