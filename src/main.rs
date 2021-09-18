use listit::builder::build_list_of_lists;
use listit::list_of_lists::ListOfLists;
use listit::program::Program;
use std::io;
use std::process;

fn main() {
    let program = Program::new();
    program.display_menu();
}
