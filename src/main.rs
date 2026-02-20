use std::env;
use std::io::{self, BufRead};

const DOG_ASCII: &str = 
"   / \\ / \\_      
   (  @  #\\___    
  /         0|    
  /   (_____/      
  /_____/";

const MAX_LENGTH: usize = 48;
const WALL_PADDING: usize = 1;

fn main() {
    //println!("MAX_LENGTH = {}", MAX_LENGTH);
    let mut words: Vec<String> = Vec::new();

    let stdin = io::stdin();
    let piped_data: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .collect();
    for idx in 0..piped_data.len() {
        words.push(piped_data[idx].to_string());
    }

    let args: Vec<String> = env::args().collect();
    for idx in 1..args.len() {
        let input = &args[idx];
        words.push(input.to_string());
    }
    
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut line_idx: usize = 0;
    lines.push(Vec::new());
    for word in &words[0..words.len()] {
        if word.len() > MAX_LENGTH {
            let mut temp: usize = 0;
            while temp < word.len() {
                lines.push(Vec::new());
                line_idx += 1;
                lines[line_idx].push(word[temp..(temp+MAX_LENGTH).min(word.len())].to_string());
                temp += MAX_LENGTH;
            }
            continue;
       } else if word.len() + lines[line_idx].len() + lines[line_idx].iter().map(|x| x.chars().count()).sum::<usize>() > MAX_LENGTH {
            lines.push(Vec::new());
            line_idx += 1;
       }
       lines[line_idx].push(word.to_string());
    }
    //println!("{:?}", lines);
    if words.len() == 0 {
        lines[line_idx].push("Woof?".to_string());
    } 
    if lines.len() == 1 {
        let left_wall_pad = std::iter::repeat(" ").take(WALL_PADDING).collect::<String>();
        println!("<{}{}{}>", left_wall_pad, lines[0].join(" "), left_wall_pad);    
    } else {
        let border = std::iter::repeat("-").take(MAX_LENGTH + WALL_PADDING).collect::<String>();
        println!(" {} ", border);
        for line in &lines[..] {
            let left_wall_pad = std::iter::repeat(" ").take(WALL_PADDING).collect::<String>();
            let right_wall_pad = std::iter::repeat(" ").take(MAX_LENGTH - line.join(" ").chars().count()).collect::<String>();
        println!("|{}{}{}|", left_wall_pad, line.join(" "), right_wall_pad);
        }
        println!(" {} ", border);
    }
    let parts = DOG_ASCII.split("\n");
    for part in parts {
        println!("{part}")
    }
}
