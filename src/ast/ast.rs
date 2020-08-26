#[path = "../lexer/lex.rs"] mod lex;

#[derive(Debug, PartialEq, Clone)]
pub enum stm{
     Let_Stm(let_stm_node),
     Ret_Stm(ret_stm_node),
     Exp_Stm(exp_stm_node),
     Stm(let_stm_node)
     
}

impl  stm{
   fn token_literal(&mut self) -> String{
       match self {
           stm::Let_Stm(s) => s.token_literal(),
           stm::Ret_Stm(s) => s.token_literal(),
           stm::Exp_Stm(s) => s.token_literal(),
           stm::Stm(s) => s.token_literal()
       }
    }

}
#[derive(Debug,PartialEq,Clone)]
pub enum exp{
     Node(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct let_stm_node{
        pub token:  super::lex::Token,
        pub name:  iden,
        pub value:  exp
}
#[derive(Debug,PartialEq,Clone)]
pub struct ret_stm_node{
    pub token: super::lex::Token,
    pub ret_value: exp 
}
#[derive(Debug,PartialEq,Clone)]
pub struct exp_stm_node{
    pub token: super::lex::Token,
    pub exp: exp
}
trait Stm {
    fn token(&mut self) -> super::lex::Token;
    fn token_literal(&mut self) -> String{
        String::from(&self.token().literal)
    }
}

impl Stm for let_stm_node{
   fn token(&mut self) -> super::lex::Token{
        return self.token.clone()
    }
   fn token_literal(&mut self) -> String{
        String::from(&self.token.literal)
    }
}
impl Stm for ret_stm_node{
   fn token(&mut self) -> super::lex::Token{
        return self.token.clone()
    }
   fn token_literal(&mut self) -> String{
        String::from(&self.token.literal)
    }
}
impl Stm for exp_stm_node{
   fn token(&mut self) -> super::lex::Token{
        return self.token.clone()
    }
   fn token_literal(&mut self) -> String{
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
    pub statements: Vec<super::ast::stm>,   
    pub errors: Vec<String>
}

impl Program{
pub fn token_literal_of_stm(stm: &super::ast::stm)-> String{
    stm.clone().token_literal()
}
pub fn token_literal(&mut self) -> String{
        if std::vec::Vec::len(&self.statements) > 0 {
           Program::token_literal_of_stm(&self.statements[0])
       }else{
            "".to_string()
        }

    }
}

fn main(){
    println!("Hi")
}