#[path = "lexer/lex.rs"] mod lex;
#[path= "lexer/lex_test.rs"] mod lex_test;

fn main() {
    println!("REPL for the Monkey Language! ");
    loop {
    let in_line = dialoguer::Input::<String>::new().with_prompt(">> ").interact().unwrap();

    let toks = lex::lex(&in_line);
    println!("Given line, loop new iter: {}", in_line);
    println!("Tokenization : {:#?}" , toks);
    }
}

