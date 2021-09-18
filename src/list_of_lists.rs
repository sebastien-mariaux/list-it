use crate::list::List;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListOfLists {
    pub lists: Vec<List>,
}

impl ListOfLists {
    fn display_titles(&self) -> String {
        self.lists.iter().map(|list| list.display_title()).collect::<Vec<String>>().join("\n")
    }

    pub fn print_titles(&self) {
        println!("{}", self.display_titles())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_titles() {
      let animals = List{title: "Animals".to_string(), id: 3, items: vec!["Cats".to_string(), "Dogs".to_string()]};
      let books = List{title: "Books".to_string(), id: 2, items: vec!["Porterhouse Blue (Tom Sharpe)".to_string(), "La mécanique du coeur (Mathias Malzieu)".to_string()]};
      let menu = ListOfLists{lists: vec![books, animals]};
      let expected = String::from("2 - Books\n3 - Animals");
      assert_eq!(expected, menu.display_titles());
    }
}