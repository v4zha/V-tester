extern crate lalrpop;
fn main() {
  lalrpop::Configuration::new()
        .always_use_colors()
         .generate_in_source_tree()
        .process().unwrap();
  tauri_build::build();
}
