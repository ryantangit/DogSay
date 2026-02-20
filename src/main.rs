use std::env;

const DOG_ASCII: &str = 
"   / \\ / \\_      
   (  @  #\\___    
  /         0|    
  /   (_____/      
  /_____/";

const MAX_LENGTH: usize = 46;

fn main() {
    println!("MAX_LENGTH = {}", MAX_LENGTH);
    let args: Vec<String> = env::args().collect();
    let mut words: Vec<String> = Vec::new();
    for idx in 1..args.len() {
        let input = &args[idx];
        words.push(input.to_string());
    }
    
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut line_idx: usize = 0;
    lines.push(Vec::new());
    for word in words {
        if word.len() > MAX_LENGTH {
            let mut temp: usize = 0;
            while temp < word.len() {
                lines.push(Vec::new());
                line_idx += 1;
                lines[line_idx].push(word[temp..(temp+MAX_LENGTH).min(word.len())].to_string());
                temp += MAX_LENGTH;
            }
       } else if word.len() + lines[line_idx].len() > MAX_LENGTH {
            line_idx += 1;
            lines.push(Vec::new())
        }
        lines[line_idx].push_str(" ");
        lines[line_idx].push_str(&word);
    }
    println!("{:?}", lines);
    let parts = DOG_ASCII.split("\n");
    for part in parts {
        println!("{part}")
    }
}


