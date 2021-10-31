use listit::program::Program;

fn main() {
    let program = Program::new(&"data.json".to_string());
    program.display_menu();
}
