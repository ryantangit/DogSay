const DOG_ASCII: &str = 
"   / \\ / \\_      
   (  @  #\\___    
  /         0|    
  /   (_____/      
  /_____/";


fn main() {
    let parts = DOG_ASCII.split("\n");
    for part in parts {
        println!("{part}")
    }
}
