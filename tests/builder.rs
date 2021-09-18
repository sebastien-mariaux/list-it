use listit::builder::build_list_of_lists;

#[test]
fn test_build_data_from_file() {
    let created = build_list_of_lists("test_data.json".to_string());
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
    let created = build_list_of_lists("missing.json".to_string());
    assert_eq!(0, created.lists.len());
}
