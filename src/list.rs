use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub title: String,
    pub id: u32,
    pub items: Vec<String>,
}

impl List {
    pub fn display_title(&self) -> String {
        format!("{} - {}", self.id, self.title)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_title() {
      let list = List{title: "Animals".to_string(), id: 3, items: vec!["Cats".to_string(), "Dogs".to_string()]};
      let expected = String::from("3 - Animals");
      assert_eq!(expected, list.display_title());
    }

    // #[test]
    // fn test_create_list(){
    //   let list = List::new("Books");
    //   assert_eq!("Books", list.name);
    // }

    // #[test]
    // fn test_add_item() {
    //   let mut list = List::new("Books");
    //   list.add("Porterhouse Blue (Tom Sharpe)");
    //   list.add("La mécanique du coeur (Mathias Malzieu)");
    //   let expected = vec!("Porterhouse Blue (Tom Sharpe)", 
    //   "La mécanique du coeur (Mathias Malzieu)");
    //   assert_eq!(expected, list.items());
    // }

    // #[test]
    // fn test_create_item() {
    //   let item = Item::new("New item");
    //   assert_eq!("New item".to_string(), item.value);
    // }
}