//function to take user input without writing 1984 lines

use std::io::{self,Write};
#[allow(unused_must_use)]

pub fn uis(x: &str) -> String {
    print!("{}",x);
    let mut prompt = String::new();
    io::stdout().flush(); //allows to take input on the same line
    io::stdin().read_line(&mut prompt).expect("Failed to read line");
    let prompt = prompt.trim(); // probably removes extra useless characters present in the string
    return prompt.to_string(); //returns the string inputed
}
