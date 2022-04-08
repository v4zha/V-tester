mod ast;
use ast::*;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub vscript); 
fn main(){
    let input="info {
         name=v4zha ; 
         desc=super ; 
        } 
    program {
         lang = python ; 
         run_env = gcc;
        }
    tests{
        (vtest)
        }"
    .into();
    let res:Box<Instruction> =vscript::InstructionsParser::new().parse(input).unwrap();
    println!("{:?}",res);
}

// lalrpop_mod!(pub test);

// fn main(){
//     let input="(vazha){input[pp,23]output[ap,bb,rrr]},(app){input[no]output[pls]}".into();
//     let res:Vec<Test> =test::TestsParser::new().parse(input).unwrap();
//     println!("{:?}",res);
// }
