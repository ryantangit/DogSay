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
    for arg in args {
        println!("{arg}")
    }
    let parts = DOG_ASCII.split("\n");
    for part in parts {
        println!("{part}")
    }
}
