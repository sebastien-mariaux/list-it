use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub title: String,
    pub index: u32,
    pub items: Vec<String>,
}

impl List {
    pub fn new(title: &str, index: u32) -> List {
        List {
            title: title.to_string(),
            items: vec![],
            index,
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn display_title(&self) -> String {
        format!("{} - {}", self.index, self.title)
    }

    pub fn display_items(&self) -> String {
        self.items.join("\n")
    }

    pub fn delete_item(&mut self, item_delete: String) {
        if let Some(index) = self.items.iter().position(|item| item == &item_delete) {
            self.items.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_title() {
        let list = List {
            title: "Animals".to_string(),
            index: 3,
            items: vec!["Cats".to_string(), "Dogs".to_string()],
        };
        let expected = String::from("3 - Animals");
        assert_eq!(expected, list.display_title());
    }

    #[test]
    fn test_display_items() {
        let list = List {
            title: "Animals".to_string(),
            index: 3,
            items: vec!["Cats".to_string(), "Dogs".to_string()],
        };
        let expected = String::from("Cats\nDogs");
        assert_eq!(expected, list.display_items());
    }

    #[test]
    fn test_create_list() {
        let list = List::new("Books", 1);
        assert_eq!("Books", list.title);
        assert_eq!(1, list.index);
        assert_eq!(Vec::<String>::new(), list.items);
    }

    #[test]
    fn test_add_item() {
        let mut list = List::new("Books", 1);
        list.add_item("Porterhouse Blue (Tom Sharpe)".to_string());
        list.add_item("La mécanique du coeur (Mathias Malzieu)".to_string());
        let expected = vec![
            "Porterhouse Blue (Tom Sharpe)",
            "La mécanique du coeur (Mathias Malzieu)",
        ];
        assert_eq!(expected, list.items);
    }

    #[test]
    fn delete_item() {
        let mut list = List {
            title: "Animals".to_string(),
            index: 3,
            items: vec!["Cats".to_string(), "Dogs".to_string()],
        };
        list.delete_item("Cats".to_string());
        assert_eq!(vec!["Dogs"], list.items);
    }
}
