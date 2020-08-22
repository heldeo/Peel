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
    fn cur_token_is(&mut self, t: super::lex::TokenType) -> bool{
        t == self.cur_token.clone().unwrap_or(super::lex::cons_eof_tok()).kind
    }
    fn peek_token_is(&mut self, t: super::lex::TokenType)-> bool{
        t == self.peek_token.clone().unwrap_or(super::lex::cons_eof_tok()).kind
    }
    fn expect_peek(&mut self, t: super::lex::TokenType)-> bool{
        if self.peek_token_is(t){
            self.next_token();
            true
        }else{
            false
        }
    }
    fn next_token(&mut self){
       self.cur_token =  self.peek_token.clone();
       self.peek_token = Some( self.l.next_token()); 
    }
    fn parse_let_stm(&mut self) -> Option<super::ast::stm>{
        if !self.expect_peek(super::lex::TokenType::IDENT){
            return None;
        }
        let node = super::ast::node{
            token: self.cur_token.clone().unwrap(),
            name: super::ast::iden{
                token: self.cur_token.clone().unwrap(),
                value: self.cur_token.clone().unwrap().literal
            },
            value: super::ast::exp::Node("".to_string())
        };
        
        while !self.cur_token_is(super::lex::TokenType::SEMICOLON){
            self.next_token();
        };
        Some(super::ast::stm::Let_Stm(node))
    }
    fn parse_stm(&mut self) -> super::ast::stm{
       match self.cur_token.clone().unwrap().kind {
           super::lex::TokenType::LET => self.parse_let_stm().unwrap(),
           _ => self.parse_let_stm().unwrap()
       } 
    }
    pub fn parse_program(&mut self) -> Option<super::ast::Program>{
        let program = ast::Program  {
            statements: vec![]
        };
        while self.cur_token.clone().unwrap_or(super::lex::cons_eof_tok()).kind != super::lex::TokenType::EOF{
                let stm = self.parse_stm();
        }
        None 
    }
}

fn main(){

}