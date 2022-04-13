use crate::vscript_dsl::{ast, vscript, dsl_errors::{ParseError, InstructionError}};
use std::{fs::File, io::Read,error::Error};
use std::process::{Command,Stdio};
use std::io::Write;
pub struct TestBuilder<'a> {
    tester_path: &'a str,
    inp_file: &'a str,
    out_file: &'a str,
    test_path: &'a str,
    instructions: Result<Option<Box<ast::Instruction>>,Box<dyn Error>>,
    compiler_opts: Vec<String>,
    run_opts: Option<String>,
}
#[derive(Debug)]
pub struct Tester {
    pub instructions: Box<ast::Instruction>,
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
    pub fn parse_file(mut self) -> Result<Self,Box<dyn Error>> {
        let mut input = String::new();
        File::open(self.tester_path)?
            .read_to_string(&mut input)?;
        let res = vscript::InstructionsParser::new().parse(&input);
        match res{
            Ok(res)=>{
                let res=res?;
                self.instructions=Ok(Some(res));
                Ok(self)
            }
            Err(_)=>{
                return Err(Box::new(ParseError::SyntaxError));
            }
        }
    }
    pub fn compiler_opts(&self, env: &ast::RunEnv) -> Vec<String> {
        let env: String = env.into();
        let env = &env[..];
        let opts = match env {
            "gcc" => vec![
                "gcc".into(),
                format!("{}/{}", self.test_path, self.inp_file),
                "-o".into(),
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
            "gcc" | "clang" => Some(format!("{}/{}", self.test_path, self.out_file,)),
            _ => None,
        };
        opts
    }
    pub fn opts(mut self) -> Result<Self,Box<dyn Error>> {
        let instructions=self.instructions.as_ref();
        match instructions{
            Ok(ins)=>{
                let env = ins.as_ref().unwrap().program.run_env;
                self.compiler_opts = self.compiler_opts(&env);
                let run_env = &self.compiler_opts.iter().nth(0).expect("Unable to fetch run Env");
                self.run_opts = self.run_opts(run_env);
                Ok(self)
            }
            Err(err)=>{
                Err(Box::new(InstructionError::RunEnvError))
            }
        }
    
    }
    pub fn build(self) -> Result<Tester,Box<dyn Error>> {
        let tester=Tester {
            instructions: self.instructions?.unwrap(),
            compiler_opts: self.compiler_opts,
            run_opts: self.run_opts,
        };
        Ok(tester)
    }
}
impl<'a> Default for TestBuilder<'a> {
    fn default() -> Self {
        Self {
            tester_path: "v_test/lv_1/tester.vscript",
            instructions: Ok(None),
            inp_file: "test.c",
            out_file: "test",
            test_path: "src/v_test",
            compiler_opts: Vec::new(),
            run_opts: None,
        }
    }
}
impl Tester {
    pub fn run_test(&self,input:Vec<String>)->Result<Output,Box<dyn Error>>{
    let mut args=self.compiler_opts.clone();
    args.remove(0);
    let res=Command::new("gcc").args(args).output()?;
    let res=Command::new(self.run_opts.as_ref().unwrap()).stdin(Stdio::piped()).spawn();
    match res{
       Ok(child)=>{
        let input=input.into_iter().reduce(|acc,val|{format!("{}\n{}",acc,val)}).unwrap();
        child.stdin.as_ref().unwrap().write(input.as_bytes())?;
        let out=child.wait_with_output()?;
        Ok(out)
       } 
       Err(err)=>{Err(Box::new(err))},
    }
    }
}

#[cfg(test)]
mod test {
    use crate::test_runner::tester::{TestBuilder,Tester};
    use std::error::Error;
    //run_opts check with test_path : )
    #[test]
    fn run_check() ->Result<(),Box<dyn Error>>{
        let tester = TestBuilder::default()
            .test_path("src/tests")
            .parse_file().unwrap()
            .opts()?
            .build()?;
        assert!(tester.run_opts == Some("src/tests/test".into()));
        Ok(())
    }
    //compiler_opts check with input file and output file : )
    #[test]
    fn compiler_check() {
        let tester = TestBuilder::default()
            .input("vazha.c")
            .output("vazha")
            .parse_file().unwrap()
            .opts().unwrap()
            .build().unwrap();
        let opts: Vec<String> = vec![
            "gcc".into(),
            "src/v_test/vazha.c".into(),
            "-o".into(),
            "src/v_test/vazha".into(),
        ];
        assert!(tester.compiler_opts == opts)
    }
    #[test]
    fn test_file(){
            let tester = TestBuilder::default()
            .test_path("v_test/lv_1")
            .input("test.c")
            .output("test")
            .parse_file().unwrap()
            .opts().unwrap()
            .build().unwrap();    
        let input=&tester.instructions.tests.test.get(0).unwrap().input;
        if let Err(err)=tester.run_test(input.to_vec()){
            panic!("Error {} ",err);
        };
    }
}
