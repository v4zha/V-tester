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
#[derive(Debug)]
pub struct Instruction {
    info: Box<Info>,
    pub program: Box<Program>,
    tests: Box<Tests>,
}
#[derive(Debug)]
pub struct Info {
    name: String,
    desc: String,
}
#[derive(Debug)]
pub struct Program {
    language: PrgLang,
    pub run_env: RunEnv,
}
#[derive(Debug)]
pub struct Tests {
    test: Vec<Test>,
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

#[derive(Debug)]
pub struct Test {
    t_name: String,
    input: Vec<String>,
    output: Vec<String>,
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
        let mut res: Vec<Test> = Vec::new();
        for t in test {
            if let Ok(val) = t {
                res.push(val);
            }
        }
        Self { test: res }
    }
}
#[derive(Debug)]
enum PrgLang {
    C,
    Python,
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Compiler {
    Gcc,
    Clang,
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Interpreter {
    Python,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
