use std::io::stdin;

/// Prompt for user input
pub fn prompt(text: &str) -> String {
    println!("{}", text);
    let mut res = String::new();
    stdin().read_line(&mut res).unwrap();

    // Yes this is an extra allocation, oh well
    res.trim().to_string()
}

/// Wait for an enter press
pub fn wait() {
    stdin().read_line(&mut String::new()).unwrap();
}
