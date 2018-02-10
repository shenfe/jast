extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/ecma-262.pest"]
struct IdentParser;

fn main() {
    let pairs = IdentParser::parse(Rule::ident_list, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.into_span().as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.into_span().as_str()), 
                _ => unreachable!()
            };
        }
    }
}