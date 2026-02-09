use std::env;

const DOG_ASCII: &str = 
"   / \\ / \\_      
   (  @  #\\___    
  /         0|    
  /   (_____/      
  /_____/";

const MAX_LENGTH: i32 = 50;

fn main() {
    println!("MAX_LENGTH = {}", MAX_LENGTH);
    let args: Vec<String> = env::args().collect();
    let mut words: Vec<String> = Vec::new();
    for idx in 1..args.len() {
        let input = &args[idx];
        words.push(input.to_string());
    }
    
    let mut lines: Vec<String> = Vec::new();
    lines.push(String::new());
    for word in words {
        //Word length is less than the current cell
        lines[0].push_str(&word);
    }
    println!("{:?}", lines);
    let parts = DOG_ASCII.split("\n");
    for part in parts {
        println!("{part}")
    }
}
