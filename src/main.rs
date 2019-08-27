extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "p21.pest"]
pub struct P21Parser;


fn main() {
    let test = include_str!("solidworks_export_simple_disk.STEP");

    let s_parse = P21Parser::parse(Rule::exchange_file, test);
    println!("{:#?}", s_parse);
    
}
