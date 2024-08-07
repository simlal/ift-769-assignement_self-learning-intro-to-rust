
fn longest_ok<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x > y {
        x
    } else {
        y
    }
}


fn main() {
    // wont conpile
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_ok(string1.as_str(), string2.as_str());
    } 
    println!("The longest string is {result}");

    // OK!
    let string1 = String::from("long string is long");
    
        {
            let string2 = String::from("xyz");
            let result = longest_ok(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        } // string2 goes out of scope here, but string1 is still valid
}