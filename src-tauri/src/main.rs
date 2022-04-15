#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod vscript_dsl;
mod test_runner;
use vscript_dsl::ast::{Tests,Test};
use test_runner::tester::{TestBuilder,Tester};
#[macro_use]
extern crate lazy_static;

lazy_static!(
  static ref VTESTER:Tester=build();
);
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tests,validate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
fn build()->Tester{
  TestBuilder::default()
            .test_path("src-tauri/v_test/lv_1")
            .input("test.c")
            .output("test")
            .parse_file().unwrap()
            .opts().unwrap()
            .build().unwrap()
}
#[tauri::command]
fn tests<'a>()->&'a Tests{
    &*VTESTER.instructions.tests
}
#[tauri::command]
fn validate(t_name:String)->Result<bool,String>{
  let test:Vec<&Test> =VTESTER.instructions.tests.test.iter().filter(|t|t.t_name==t_name).collect();
  let res=VTESTER.run_test(&test[0].input);
  if let Err(err)=res{
    return Err(format!("Error in geting output {:?}",err))
  }
  let test_out=test[0].output.clone();

  let res=Tester::validate_test(res.unwrap(),test_out);
  match res{
    Ok(val)=>Ok(val),
    Err(err)=>Err(format!("Error in validating {:?}",err))
  }
}

