use std::collections::HashMap;
fn main() {
    let mut phone_book = HashMap::new();
    let first_entry = String::from("simon");
    let first_num = 123456;

    phone_book.insert(first_entry, first_num);
    println!("first_entry: {first_entry}");
}
