mod list;
mod list_of_lists;

use list_of_lists::ListOfLists;
use serde_json::Result;
use std::fs;



fn main() {
    let lists = build_list_of_lists("data.json".to_string());
    dbg!(lists);
    
}

pub fn build_list_of_lists( data_file: String) -> ListOfLists {
    let raw_data = import_data(data_file);
    let lists = build_from_data(raw_data);
    match lists {
        Ok(l) => l,
        _ => ListOfLists{lists: vec![]}
    }
}

fn import_data( data_file: String) -> String {
    let data = fs::read_to_string(data_file);
    match data {
        Ok(d) => d,
        _ => String::default()
    }
}

fn build_from_data( raw_data: String) -> Result<ListOfLists> {
    let lists: ListOfLists = serde_json::from_str(raw_data.as_str())?;
    Ok(lists)
}


