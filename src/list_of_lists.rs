#![allow(unused)]

use crate::list::List;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListOfLists {
    pub lists: Vec<List>,
}

impl ListOfLists {
    pub fn display_titles(&self) -> String {
        self.lists
            .iter()
            .map(|list| list.display_title())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn create_list(&mut self, title: String) {
        let next_index = self.next_index();
        let new_list = List::new(title.as_str(), next_index);
        self.lists.push(new_list);
    }

    pub fn add_item_to_list(&mut self, index: u32, item: String) {
        let list = self.lists.iter_mut().find(|list| list.index == index);
        match list {
            Some(l) => l.add_item(item),
            None => println!("Oh my... We couldn't find this list"),
        }
    }

    pub fn display_list(&self, index: u32) -> String {
        let list = self.lists.iter().find(|list| list.index == index);
        match list {
            Some(l) => format!(
                "\n-----------------\n{}\n-----------------\n{}",
                l.title,
                l.display_items()
            ),
            None => String::default(),
        }
    }

    fn next_index(&self) -> u32 {
        let maxi = self.lists.iter().map(|l| l.index).max();
        match maxi {
            Some(x) => x + 1,
            None => 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup_list_of_list() -> ListOfLists {
        let animals = List {
            title: "Animals".to_string(),
            index: 2,
            items: vec!["Cats".to_string(), "Dogs".to_string()],
        };
        let books = List {
            title: "Books".to_string(),
            index: 1,
            items: vec![
                "Porterhouse Blue (Tom Sharpe)".to_string(),
                "La mécanique du coeur (Mathias Malzieu)".to_string(),
            ],
        };
        ListOfLists {
            lists: vec![books, animals],
        }
    }

    #[test]
    fn display_titles() {
        let list_of_lists = setup_list_of_list();
        let expected = String::from("1 - Books\n2 - Animals");
        assert_eq!(expected, list_of_lists.display_titles());
    }

    #[test]
    fn get_next_index() {
        let list_of_lists = setup_list_of_list();
        assert_eq!(3, list_of_lists.next_index());
    }

    #[test]
    fn get_first_index() {
        let list_of_lists = ListOfLists { lists: vec![] };
        assert_eq!(1, list_of_lists.next_index());
    }
    #[test]
    fn create_new_list() {
        let mut list_of_lists = setup_list_of_list();
        assert_eq!(2, list_of_lists.lists.len());
        list_of_lists.create_list("Things to do".to_string());
        assert_eq!(3, list_of_lists.lists.len());
    }

    #[test]
    fn add_item_to_list() {
        let mut list_of_lists = setup_list_of_list();
        list_of_lists.add_item_to_list(2, "Chameleons".to_string());
        assert_eq!(
            vec![
                "Cats".to_string(),
                "Dogs".to_string(),
                "Chameleons".to_string()
            ],
            list_of_lists.lists[1].items
        );
    }

    #[test]
    fn not_add_item_to_list_index_not_found() {
        let mut list_of_lists = setup_list_of_list();
        list_of_lists.add_item_to_list(10, "Chameleons".to_string());
        assert_eq!(
            vec!["Cats".to_string(), "Dogs".to_string()],
            list_of_lists.lists[1].items
        );
    }

    #[test]
    fn display_list() {
        let list_of_lists = setup_list_of_list();
        let text = "\n-----------------\nAnimals\n-----------------\nCats\nDogs";
        let expected = String::from(text);
        assert_eq!(expected, list_of_lists.display_list(2));
    }
}
