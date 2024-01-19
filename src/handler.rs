use crate::db;
use crate::utils;
use tabled::settings::Style;

pub async fn add() {
    bunt::println!("exec `add` command...");
    let key = inquire::Text::new("type key: ")
        .with_help_message("please type any character")
        .prompt()
        .unwrap();
    let value = inquire::Text::new("type value: ")
        .with_help_message("please type any value")
        .prompt()
        .unwrap();
    let hash = super::utils::random_hash();
    db::add(key.clone(), value.clone(), hash).await;
    bunt::println!("entry add success: {$green}{}{/$}", key);
    bunt::println!("value: {$yellow}{}{/$}", key);
}

pub async fn list() {
    bunt::println!("exec `list` command...");
    let entries = db::list().await.unwrap();
    let mut builder = utils::get_table();
    entries.iter().for_each(|e| {
        builder.push_record(vec![
            (&e.key).to_string(),
            (&e.value).to_string(),
            (&e.hash).to_string(),
            chrono::DateTime::parse_from_rfc3339(&e.created_at)
                .unwrap()
                .format("%a %b %e %T %Y")
                .to_string(),
        ]);
    });
    let table = builder.build().with(Style::rounded()).to_string();
    bunt::println!("{}", table);
}

pub async fn delete() {
    bunt::println!("exec `list` command...");
    let key = inquire::Text::new("type key: ")
        .with_help_message("type any character")
        .prompt()
        .unwrap();
    db::delete(key.clone()).await.unwrap();
    bunt::println!("delete entry key success: {$red}{}{/$}", key);
}

pub async fn get() {
    bunt::println!("exec `get` command...");
    let key = inquire::Text::new("type key: ")
        .with_help_message("type any character")
        .prompt()
        .unwrap();
    let entry = db::get(key).await.unwrap();
    bunt::println!("entry key: {$green}{}{/$}", entry.key);
    bunt::println!("entry value: {$yellow}{}{/$}", entry.value);
}

pub async fn search() {
    bunt::println!("exec `search` command...");
    let keys = db::list_keys().await.unwrap();
    let key = inquire::Select::new("Select Key: ", keys)
        .with_page_size(10)
        .prompt()
        .unwrap();
    let entry = super::db::get(key).await.unwrap();
    bunt::println!("entry key: {$green}{}{/$}", entry.key);
    bunt::println!("entry value: {$yellow}{}{/$}", entry.value);
}
