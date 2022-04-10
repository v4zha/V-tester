use crate::vscript_dsl::{vscript,ast};
use std::{fs::File, io::Read};

pub struct TestBuilder<'a> {
    tester_path: &'a str,
    inp_file: &'a str,
    out_file: &'a str,
    test_path: &'a str,
    instructions: Option<Box<ast::Instruction>>,
    compiler_opts: Vec<String>,
    run_opts: Option<String>,
}
#[derive(Debug)]
pub struct Tester {
    instructions: Box<ast::Instruction>,
    compiler_opts: Vec<String>,
    run_opts: Option<String>,
}

impl<'a> TestBuilder<'a> {
    pub fn path(mut self, path: &'a str) -> Self {
        self.tester_path = path;
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
        File::open(self.tester_path)
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let res = vscript::InstructionsParser::new().parse(&input).unwrap();
        let res = res.unwrap();
        self.instructions = Some(res);
        self
    }
    pub fn compiler_opts(&self, env: &ast::RunEnv) -> Vec<String> {
        let env: String = env.into();
        let env = &env[..];
        let opts = match env {
            "gcc" => vec![
                "gcc".into(),
                format!("{}/{}", self.test_path, self.inp_file),
                "-O".into(),
                format!("{}/{}", self.test_path, self.out_file),
            ],
            "clang" => vec![
                "clang".into(),
                format!("{}/{}", self.test_path, self.inp_file),
                "-o".into(),
                format!("{}/{}", self.test_path, self.out_file),
            ],
            "python" => vec!["python3".into(), self.inp_file.into()],
            _ => Vec::new(),
        };
        opts
    }
    pub fn run_opts(&self, env: &str) -> Option<String> {
        let opts = match env {
            "gcc" | "clang" => Some(format!(
                "{}{}",
                self.test_path,
                self.out_file,
            )),
            _ => None,
        };
        opts
    }
    pub fn opts(mut self)->Self{
        let env=&self.instructions.as_ref().unwrap().program.run_env;
        self.compiler_opts=self.compiler_opts(env);
        let run_env=&self.compiler_opts.iter().nth(0).unwrap();
        self.run_opts=self.run_opts(run_env);
        self
    }
    pub fn build(self) -> Tester {
        Tester {
            instructions: self.instructions.unwrap(),
            compiler_opts: self.compiler_opts,
            run_opts: self.run_opts,
        }
    }
}
impl<'a> Default for TestBuilder<'a> {
    fn default() -> Self {
        Self {
            tester_path:"./tester.vscript",
            instructions:None,
            inp_file:"test.c",
            out_file:"test",
            test_path:"tests",
            compiler_opts:Vec::new(),
            run_opts:None
        }
    }
}
