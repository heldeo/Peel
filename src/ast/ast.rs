#[path = "../lexer/lex.rs"] mod lex;

#[derive(Debug, PartialEq, Clone)]
pub enum stm{
     Let_Stm(node),
     Stm(node)
}

#[derive(Debug,PartialEq,Clone)]
pub enum exp{
     Node(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct node{
        pub token:  super::lex::Token,
        pub name:  iden,
        pub value:  exp
}

impl node{
    fn statment_node(&mut self){
        ()
    }
    pub fn token_literal(& self) -> String{
        String::from(&self.token.literal)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct iden{
    pub token:  super::lex::Token,
    pub value:  String
}
impl iden{
    fn expression_node(&mut self){
        ()
    }
    pub fn token_literal(&mut self) -> String{
        String::from(self.token.literal.clone())
    }
}

pub struct Program {
    pub statements: Vec<super::ast::stm>
}

impl Program{
pub fn token_literal(&mut self) -> String{
        if std::vec::Vec::len(&self.statements) > 0 {
            let stm = match self.statements[0].clone() {
              stm::Let_Stm(s) => s, 
              stm::Stm(s) => s
            };
            stm.token_literal()
       }else{
            "".to_string()
        }

    }
}

fn main(){
    println!("Hi")
}