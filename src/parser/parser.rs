#[path ="../ast/ast.rs"] mod ast;
#[path = "../lexer/lex.rs"] mod lex;

pub struct parser{
    l:  super::lex::Lexer,
    cur_token:  Option<super::lex::Token>,
    peek_token:   Option<super::lex::Token>
}

pub fn cons_parser(lexer: super::lex::Lexer) ->  parser{
    let mut parser = parser {
            l: lexer,
            cur_token:  None,
            peek_token:  None 
        };
        parser.next_token();
        parser
 }

impl parser{
    fn next_token(&mut self){
       self.cur_token =  self.peek_token.clone();
       self.peek_token = Some( self.l.next_token()); 
    }

    pub fn parse_program(&mut self) -> Option<super::ast::Program>{
        
       None 
        //super::ast::Program{
         //   statements: vec![ super::ast::stm::Stm_Node()]
        //}
        //
    }
}

fn main(){

}