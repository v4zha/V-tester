use serde::Serialize;

use crate::vscript_dsl::dsl_errors::{InstructionError, ParseError};
use std::vec::Vec;
/*
    NOTE: newline=';'
    info{
        name,
        discription,
    }
    program{
        language,
        run_env,
    }
    tests{
        (test_name){
            input[1,2,3..]
            output[yes;it was found]
        },
        (test_2){
            input[2,3,5]
            output[No;not found]
        }
    }
*/
pub struct Instruction {
    pub info: Box<Info>,
    pub program: Box<Program>,
    pub tests: Box<Tests>,
}
#[derive(Serialize)]
pub struct Info {
    name: String,
    desc: String,
}
pub struct Program {
    language: PrgLang,
    pub run_env: RunEnv,
}
#[derive(Serialize)]
pub struct Tests {
    pub test: Vec<Test>,
}

impl Info {
    pub fn new(name: String, desc: String) -> Self {
        Self { name, desc }
    }
}
impl Program {
    pub fn new(language: &str, run_env: &str) -> Self {
        let language: String = language.split_whitespace().collect();
        let lang: Result<PrgLang, InstructionError> = match language.as_str() {
            "c" => Ok(PrgLang::C),
            "python" => Ok(PrgLang::Python),
            _ => Err(InstructionError::LanguageError),
        };
        let lang = lang.unwrap();
        let env: Result<RunEnv, InstructionError> = match run_env {
            "python" => Ok(RunEnv::Interpreter(Interpreter::Python)),
            "gcc" => Ok(RunEnv::Compiler(Compiler::Gcc)),
            "clang" => Ok(RunEnv::Compiler(Compiler::Clang)),
            _ => Err(InstructionError::RunEnvError),
        };
        let env = env.unwrap();
        env.check_env(&lang).unwrap();
        Self {
            language: lang,
            run_env: env,
        }
    }
}

impl Instruction {
    pub fn new(info: Box<Info>, program: Box<Program>, tests: Box<Tests>) -> Self {
        Self {
            info,
            program,
            tests,
        }
    }
}

#[derive(Serialize)]
pub struct Test {
    pub t_name: String,
    pub input: Vec<String>,
    pub output: Vec<String>,
}
impl Test {
    pub fn new(t_name: String, input: Vec<String>, output: Vec<String>) -> Self {
        Self {
            t_name,
            input,
            output,
        }
    }
}

impl Tests {
    pub fn new(test: Vec<Result<Test, ParseError>>) -> Self {
        let res:Vec<Test> = test.into_iter().filter(|r|r.is_ok()).map(|r|r.unwrap()).collect();
        Self { test: res }
    }
}
enum PrgLang {
    C,
    Python,
}
#[derive( Copy, Clone, PartialEq, PartialOrd)]
pub enum Compiler {
    Gcc,
    Clang,
}
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Interpreter {
    Python,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum RunEnv {
    Compiler(Compiler),
    Interpreter(Interpreter),
}
impl RunEnv {
    fn check_env(self, lang: &PrgLang) -> Result<(), InstructionError> {
        match lang {
            PrgLang::C
                if self == RunEnv::Compiler(Compiler::Clang)
                    || self == RunEnv::Compiler(Compiler::Gcc) =>
            {
                Ok(())
            }
            PrgLang::Python if self == RunEnv::Interpreter(Interpreter::Python) => Ok(()),
            _ => Err(InstructionError::RunEnvError),
        }
    }
}
impl Into<String> for &RunEnv {
    fn into(self) -> String {
        match self {
            RunEnv::Compiler(Compiler::Clang) => "clang".into(),
            RunEnv::Compiler(Compiler::Gcc) => "gcc".into(),
            RunEnv::Interpreter(Interpreter::Python) => "python".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vscript_dsl::{dsl_errors::ParseError, vscript};
    use std::{fs::File, io::Read,error::Error};
    //file parse fn
    fn parse(path:&str)->Result<(), Box<dyn Error>>{
            let mut input = String::new();
        File::open(path)?
            .read_to_string(&mut input)?;
        let res = vscript::InstructionsParser::new().parse(&input);
        match res{
            Ok(res)=>{
                let _res=res?;
                Ok(())
            }
            Err(_)=>{
                return Err(Box::new(ParseError::SyntaxError));
            }
        }
    }
    #[test]
    //check valid parse file
    fn valid_check() {
        let path = "v_test/lv_1/tester.vscript";
        let res = parse(path);
        if let Err(err) = res {
            panic!("Erro : {:?}", err);
        }
    }
    #[test]
    //check invalid parse file
    fn invalid_check() {
        let path = "v_test/invalid/inv1.vscript";
        let res = parse(path);
        if let Ok(_) = res {
            panic!("Invalid file Parsed : )")
        }
    }
}
