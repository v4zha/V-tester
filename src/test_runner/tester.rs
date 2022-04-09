use crate::vscript_dsl::{ast, vscript};
use std::{fs::File, io::Read, iter::Inspect};

pub struct TestBuilder<'a> {
    path: &'a str,
    inp_file: &'a str,
    out_file: &'a str,
    test_path: &'a str,
    instructions: Box<ast::Instruction>,
    compiler_opts: Vec<String>,
    run_opts: Option<String>,
}
pub struct Tester{
    instructions: Box<ast::Instruction>,
    compiler_opts: Vec<String>,
    run_opts: Option<String>,
}

impl<'a> TestBuilder<'a> {
    pub fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }
    pub fn input(mut self, input: &'a str) -> Self {
        self.inp_file = input;
        self
    }
    pub fn output(mut self, output: &'a str) -> Self {
        self.out_file = output;
        self
    }
    pub fn test_path(mut self, test_path: &'a str) -> Self {
        self.test_path = test_path;
        self
    }
    pub fn parse_file(mut self) -> Self {
        let mut input = String::new();
        File::open(self.path)
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let res = vscript::InstructionsParser::new().parse(&input).unwrap();
        let res = res.unwrap();
        self.instructions = res;
        self
    }
    pub fn compiler_opts(mut self, env: &ast::RunEnv) -> Self {
        let env: String = env.into();
        let env = &env[..];
        let opts = match env {
            "gcc" => vec![
                "gcc".into(),
                self.inp_file.into(),
                "-O".into(),
                format!("{}{}", self.test_path, self.out_file),
            ],
            "clang" => vec![
                "clang".into(),
                self.inp_file.into(),
                "-o".into(),
                format!("{}{}", self.test_path, self.out_file),
            ],
            "python" => vec!["python3".into(), self.inp_file.into()],
            _ => Vec::new(),
        };
        self.compiler_opts = opts;
        self
    }
    pub fn run_opts(mut self, env: &str) -> Self {
        let opts = match env {
            "gcc" | "clang" => Some(format!("{}{}", self.test_path, self.out_file)),
            _ => None,
        };
        self.run_opts = opts;
        self
    }
    pub fn build(self)->Tester{
        Tester { instructions:self.instructions, compiler_opts:self.compiler_opts, run_opts:self.run_opts }
    }
}

