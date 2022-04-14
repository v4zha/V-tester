#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod vscript_dsl;
mod test_runner;
use vscript_dsl::ast::Tests;
use test_runner::tester::TestBuilder;
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tests])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn tests()->Tests{
      let tester = TestBuilder::default()
            .test_path("v_test/lv_1")
            .input("test.c")
            .output("test")
            .parse_file().unwrap()
            .opts().unwrap()
            .build().unwrap();
      *tester.instructions.tests 
}