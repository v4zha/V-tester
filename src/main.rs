mod test_runner;
mod vscript_dsl;
use test_runner::tester::{TestBuilder, Tester};
fn main(){
    let tester=TestBuilder::default().parse_file().opts().build();
    println!("{:?}",tester);                
}
