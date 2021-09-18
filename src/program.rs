use crate::builder::build_list_of_lists;
use crate::list_of_lists::ListOfLists;
use std::io;
use std::process;

pub struct Program {
    list_of_lists: ListOfLists,
}

impl Program {
    pub fn new() -> Self {
        let list_of_lists = build_list_of_lists("data.json".to_string());
        Program { list_of_lists }
    }

    pub fn display_menu(self) -> Self {
        println!("\n------------------------");
        println!("Here are all yours lists:");
        println!("------------------------");
        println!("{}", &self.list_of_lists.display_titles());
        println!("------------------------");
        self.select_list()
    }

    fn select_list(mut self) -> Self {
        println!("Enter a list index to see the details");
        println!("Actions:a - add a new list / e - exit");
        let input = prompt();
        self = match input.as_str() {
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
        };
        self
    }

    fn display_list(mut self, index: u32) -> Self {
        println!("{}", self.list_of_lists.display_list(index));
        self = self.get_action(index);
        self
    }

    fn get_action(mut self, list_index: u32) -> Self {
        println!("------------------------");
        println!("a - Add an item / b - back to menu / e - exit");
        self = loop {
            let action = prompt();
            self = self.do_list_action(action, list_index);
            break (self);
        };
        self
    }

    fn do_list_action(mut self, action: String, list_index: u32) -> Self {
        match action.chars().next().unwrap() {
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
        println!("{}", self.list_of_lists.display_list(list_index));
        self.list_of_lists.save_data(&"data.json".to_string());
        self
    }

    fn create_list(mut self) -> Self {
        println!("Enter the title of your new list:");
        let title = prompt();
        let index = self.list_of_lists.create_list(title);
        self.list_of_lists.save_data(&"data.json".to_string());

        self = self.display_list(index);
        self
    }
}

// fn prompt_number() -> u32 {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .ok()
//         .expect("Couldn't read line");
//     input.split("\n").next().unwrap().parse::<u32>().unwrap()
// }

fn prompt() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
    input.split("\n").next().unwrap().to_string()
}
