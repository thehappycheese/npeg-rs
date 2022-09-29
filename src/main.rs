mod ops;
use ops::ParserOperator;
use ops::Parser;






fn main() {
    let test_string = "aaa(";
    let lit_a = ParserOperator::literal("a");
    let lit_b = ParserOperator::literal("a()");
    let qtt = ParserOperator::quantity(lit_a.clone(), 1, Some(3));
    let lah = ParserOperator::lookahead(qtt.clone(), ParserOperator::literal("("), true);
    let reg = ParserOperator::regex("^[0-9]");
    let seq = ParserOperator::sequence(vec![qtt.clone(), reg, lit_b.clone()]);
    let alt = ParserOperator::alternation(vec![seq, lit_b]);
    println!("{:?}", lah.parse(test_string, 0));

    //let re = Regex::new("^[ab]").unwrap();
    //println!("{:?}", re.find(&"tac"[1..]));
}
