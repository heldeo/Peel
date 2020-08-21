#[path = "../lexer/lex.rs"] mod lex;

#[derive(Debug, PartialEq, Clone)]
enum stm {
    Node(String)
}

#[derive(Debug,PartialEq,Clone)]
enum exp{
    Node(String)
}

struct let_stm<'a>{
        token: &'a lex::Token,
        name: &'a iden<'a>,
        value: &'a exp
}
impl<'a> let_stm<'a>{
    fn statment_node(&mut self){
        ()
    }
    fn token_literal(&'a self) -> String{
        String::from(&self.token.literal)
    }
}

struct iden<'a>{
    token: &'a lex::Token,
    value: &'a String
}
impl<'a> iden<'a>{
    fn expression_node(&mut self){
        ()
    }
    fn token_literal(&'a self) -> String{
        String::from(&self.token.literal)
    }
}

/*
enum Nodes  {
    Statement(String),
    Expression(String)
}
*/

struct Program {
    statements: Vec<stm>
}

impl Program{
fn token_literal(&mut self) -> String{
        if std::vec::Vec::len(&self.statements) > 0 {
            let stm::Node(literal) = self.statements[0].clone();
            literal
       }else{
            "".to_string()
        }

    }
}

fn main(){
    println!("Hi")
}