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
