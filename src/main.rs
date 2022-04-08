mod vscript_dsl;
use vscript_dsl::ast::*;
use vscript_dsl::vscript;
use std::{fs::File, io::Read};

fn main(){
    let mut input=String::new();
    File::open("./tests/tester.vscript").unwrap().read_to_string(&mut input).unwrap();
    let res:Box<Instruction> =vscript::InstructionsParser::new().parse(&input).unwrap();
    println!("{:?}",res);
}
