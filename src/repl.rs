
const  PROMPT: &str = ">> ";

fn main(){
    println!("start");
    loop {
    print!(">> ");
    let in_line = std::io::stdin().read_line(&mut String::new()).unwrap();
    println!("Given line, loop new iter: {}", in_line)
    }
}