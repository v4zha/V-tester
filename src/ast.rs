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
pub struct Instruction{
   info:Box<Info>,
   program:Box<Program>,
   tests: Box<Tests>,
}
#[derive(Debug)]
pub struct Info{
    name:String,
    desc:String,
}
#[derive(Debug)]
pub struct Program{
    language:String,
    compiler:String,
}
#[derive(Debug)]
pub struct Tests{
    test:Vec<Test>,
}

impl Info{
    pub fn new(name:String,desc:String)->Self{
        Self{name,desc}
    }
}
impl Program{
    pub fn new(language:String,compiler:String)->Self{
        Self{language,compiler}
    }
}

impl Instruction{
    pub fn new(info:Box<Info>,program:Box<Program>,tests:Box<Tests>)->Self{
        Self{info,program,tests}
    }
}

#[derive(Debug)]
pub struct Test{
    t_name:String,
    input:Vec<String>,
    output:Vec<String>,
}
impl Test{
    pub fn new(t_name:String,input:Vec<String>,output:Vec<String>)->Self{
        Self{t_name,input,output}
    }
}

impl Tests{
    pub fn new(test:Vec<Test>)->Self{
        Self{test}
    }
}