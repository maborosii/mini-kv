use std::vec;

use rand::Rng;
use tabled::builder::Builder;
// use tabled::builder::Builder;

pub fn suggester(val: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let suggestions = ["get", "set", "delete", "list", "search", "help", "exit"];

    let val_lower = val.to_lowercase();

    Ok(suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(*s))
        .collect())
}

pub fn random_hash() -> String {
    let mut rng = rand::thread_rng();
    let rand_string = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
        .take(6)
        .collect();
    rand_string
}

pub fn get_table() -> tabled::builder::Builder {
    let mut builder = Builder::default();
    // builder.set_header(vec![
    //     "Key".to_string(),
    //     "Value".to_string(),
    //     "Hash".to_string(),
    //     "Create At".to_string(),
    // ]);
    builder.push_record(vec!["Key", "Value", "Hash", "Create At"]);
    builder
}
