#[path = "../lexer/lex.rs"] mod lex;

#[derive(Debug, PartialEq, Clone)]
pub enum stm<'a>{
     Let_Node(node<'a>),
     Stm_Node(node<'a>)
}

#[derive(Debug,PartialEq,Clone)]
pub enum exp{
     Node(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct node<'a>{
        token: &'a lex::Token,
        pub name: &'a iden<'a>,
        pub value: &'a exp
}

impl<'a> node<'a>{
    fn statment_node(&mut self){
        ()
    }
    pub fn token_literal(&'a self) -> String{
        String::from(&self.token.literal)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct iden<'a>{
    pub token: &'a lex::Token,
    pub value: &'a String
}
impl<'a> iden<'a>{
    fn expression_node(&mut self){
        ()
    }
    pub fn token_literal(&'a self) -> String{
        String::from(&self.token.literal)
    }
}

pub struct Program<'a> {
    pub statements: Vec<stm<'a>>
}

impl<'a> Program<'a>{
pub fn token_literal(&mut self) -> String{
        if std::vec::Vec::len(&self.statements) > 0 {
            let stm = match self.statements[0].clone() {
              stm::Let_Node(s) => s, 
              stm::Stm_Node(s) => s
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