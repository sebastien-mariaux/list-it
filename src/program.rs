use crate::builder::build_list_of_lists;
use crate::list;
use crate::list_of_lists::ListOfLists;
use std::io;
use std::process;

pub struct Program {
    list_of_lists: ListOfLists,
    data_file: String,
}

impl Program {
    const SEPARATOR: &'static str = "--------------------";

    pub fn new(data_file: &str) -> Self {
        let list_of_lists = build_list_of_lists(data_file);
        Program {
            list_of_lists,
            data_file: data_file.to_owned(),
        }
    }

    pub fn display_menu(self) -> Self {
        let menu = self.build_menu();
        println!("{}", menu);
        self.select_list()
    }

    fn build_menu(&self) -> String {
        format!(
            "\n{sep}\nHere are all yours lists:\n{sep}\n{titles}\n{sep}",
            sep = Program::SEPARATOR,
            titles = &self.list_of_lists.display_titles()
        )
    }

    fn select_list(mut self) -> Self {
        println!("Enter a list index to see the details");
        println!("Actions:a - add a new list / e - exit");
        let input = prompt();
        self = self.do_menu_action(input);
        self
    }

    fn do_menu_action(mut self, input: String) -> Self {
        match input.as_str() {
            "a" => {
                self = self.create_list();
                self
            }
            "e" => {
                println!("Goodbye!");
                process::exit(1);
            }
            x => {
                let index = x.parse::<u32>();
                self = match index {
                    Ok(idx) => {
                        self = self.display_list(idx);
                        self
                    }
                    _ => {
                        println!("Wut?");
                        self = self.select_list();
                        self
                    }
                };
                self
            }
        }
    }

    fn display_list(mut self, index: u32) -> Self {
        let text = self.list_of_lists.display_list(index);
        match text {
            Some(x) => {
                println!("{}", x);
                self = self.get_action(index);
                self
            }
            None => {
                println!("List does not exist!");
                self = self.select_list();
                self
            }
        }
    }

    fn get_action(mut self, list_index: u32) -> Self {
        println!("------------------------");
        println!("a - Add an item / b - back to menu / e - exit");
        let action = prompt();
        self = self.handle_user_input_for_list(action, list_index);
        self
    }

    fn handle_user_input_for_list(self, input: String, list_index: u32) -> Self {
        let input = input.chars().next();
        match input {
            Some(action) => self.handle_list_action(action, list_index),
            None => {
                println!("Wut?");
                self.get_action(list_index)
            }
        }
    }

    fn handle_list_action(mut self, action: char, list_index: u32) -> Self {
        match action {
            'a' => {
                self = self.add_item_to_list(list_index);
                self.get_action(list_index)
            }
            'b' => self.display_menu(),
            'e' => {
                println!("Goodbye!");
                process::exit(1);
            }
            _ => {
                println!("Wut?");
                self.get_action(list_index)
            }
        }
    }

    fn add_item_to_list(mut self, list_index: u32) -> Self {
        println!("Enter the new item:");
        let item = prompt();
        self.list_of_lists.add_item_to_list(list_index, item);
        self.list_of_lists.save_data(&self.data_file.to_string());
        self = self.display_list(list_index);
        self
    }

    fn create_list(mut self) -> Self {
        println!("Enter the title of your new list:");
        let title = prompt();
        let index = self.list_of_lists.create_list(title);
        self.list_of_lists.save_data(&self.data_file.to_string());

        self = self.display_list(index);
        self
    }
}

fn prompt() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");
    input.split('\n').next().unwrap().to_string()
}

mod tests {
    use super::Program;

    #[test]
    fn build_menu() {
        let program = Program::new(&"tests/test_data.json".to_string());
        let expected = "\n--------------------\n\
        Here are all yours lists:\n\
        --------------------\n\
        1 - Animaux\n\
        2 - Marques de voitures\n\
        3 - Courses\n\
        --------------------";
        assert_eq!(expected, program.build_menu());
    }

    #[test]
    fn build_menu_from_empty_data() {
        let program = Program::new(&"tests/missing_data.json".to_string());
        let expected = "\n--------------------\n\
        Here are all yours lists:\n\
        --------------------\n\n\
        --------------------";
        assert_eq!(expected, program.build_menu());
    }
}
