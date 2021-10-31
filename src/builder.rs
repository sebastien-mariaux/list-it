use crate::list_of_lists::ListOfLists;
use serde_json::Result;
use std::fs;

pub fn build_list_of_lists(data_file: &str) -> ListOfLists {
    let raw_data = import_data(data_file);
    let lists = build_from_data(raw_data);
    match lists {
        Ok(l) => l,
        _ => ListOfLists { lists: vec![] },
    }
}

fn import_data(data_file: &str) -> String {
    let data = fs::read_to_string(data_file);
    match data {
        Ok(d) => d,
        _ => String::default(),
    }
}

fn build_from_data(raw_data: String) -> Result<ListOfLists> {
    let lists: ListOfLists = serde_json::from_str(raw_data.as_str())?;
    Ok(lists)
}

mod tests {

    use super::*;

    #[test]
    fn test_build_data_from_file() {
        let created = build_list_of_lists(&"tests/test_data.json".to_string());
        assert_eq!(3, created.lists.len());
        assert_eq!(
            vec![
                "Mustang".to_string(),
                "Porsche".to_string(),
                "Ferrari".to_string(),
                "Lamborgini".to_string()
            ],
            created.lists[1].items
        );
    }

    #[test]
    fn test_init_data() {
        let created = build_list_of_lists(&"missing.json".to_string());
        assert_eq!(0, created.lists.len());
    }
}
