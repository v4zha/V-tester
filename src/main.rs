mod ast;
use ast::*;
use std::{fs::File, io::Read};
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub vscript); 
fn main(){
    let mut input=String::new();
    File::open("./tests/tester.vscript").unwrap().read_to_string(&mut input).unwrap();
    let res:Box<Instruction> =vscript::InstructionsParser::new().parse(&input).unwrap();
    println!("{:?}",res);
}
