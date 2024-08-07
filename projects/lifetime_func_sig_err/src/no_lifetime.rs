fn longest_err(x: &str, y: &str) -> Result<&str, &str> {
    if x.len() > y.len() {
        Ok(x)
    } else if x.len() < y.len() {
        Ok(y)
    } else {
        Err("Strings have same length")
    }
}

fn main() {
    let long_s: String = String::from("long string is long");
    let short_s: &str = "shortest";
    
    let err = longest_err(long_s.as_str(), short_s);
    println!("The longest string is: {}", err);
}
