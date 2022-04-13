use std::{error::Error,};
use std::fmt;
#[derive(Debug)]
pub enum InstructionError {
    RunEnvError,
    LanguageError,
}
#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
    FieldError(String),
}
impl Error for InstructionError{}
impl Error for ParseError{}

impl fmt::Display for InstructionError{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
        use InstructionError::*;
        match self{
            RunEnvError=>write!(f,"Run Env error"),
            LanguageError=>write!(f,"Language Error"),
        }
    }
}
impl fmt::Display for ParseError{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        use ParseError::*;
        match self{
            SyntaxError =>write!(f,"Syntax Error"),
            FieldError(field)=>write!(f,"Error in Field {} ",field),
        }
    }
}
