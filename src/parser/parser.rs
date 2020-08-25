#[path ="../ast/ast.rs"] mod ast;
#[path = "../lexer/lex.rs"] mod lex;

pub struct parser{
    l:  super::lex::Lexer,
    cur_token:  Option<super::lex::Token>,
    peek_token:   Option<super::lex::Token>,
    errors: Vec<String>
}

pub fn cons_parser(lexer: super::lex::Lexer) ->  parser{
    let mut parser = parser {
            l: lexer,
            cur_token:  None,
            peek_token:  None, 
            errors: vec![]
        };
        parser.next_token();
        parser
 }

impl parser{
    fn cur_token_is(&mut self, t: super::lex::TokenType) -> bool{
        if self.cur_token == None {
            false
        } else{
            t == self.cur_token.clone().unwrap().kind
        }
 }
    fn peek_token_is(&mut self, t: super::lex::TokenType)-> bool{
        t == self.peek_token.clone().unwrap_or(super::lex::cons_eof_tok()).kind
    }
    fn expect_peek(&mut self, t: super::lex::TokenType)-> bool{
        if self.peek_token_is(t.to_owned()){
            self.next_token();
            true
        }else{
            self.peek_error(t);
            false
        }
    }
    fn next_token(&mut self){
       self.cur_token =  self.peek_token.clone();
       self.peek_token = Some( self.l.next_token()); 
    }
    fn parse_let_stm(&mut self) -> Option<super::ast::stm>{
        let cur_token = self.cur_token.clone().unwrap().clone();

        if !self.expect_peek(super::lex::TokenType::IDENT){
            return None;
        }

        // current token here is identifier/
        //expect_peek sideffect of moving token head

        let node = super::ast::let_stm_node{
            token: cur_token,
            name: super::ast::iden{
                token: self.cur_token.clone().unwrap(),
                value: self.cur_token.clone().unwrap().literal
            },
            value: super::ast::exp::Node("".to_string())
        };
       
        if !self.expect_peek(super::lex::TokenType::ASSIGN){
            return None
        }
        while !self.cur_token_is(super::lex::TokenType::SEMICOLON){
            self.next_token();
        };
        Some(super::ast::stm::Let_Stm(node))
    }
    fn parse_return_stm(&mut self) -> Option<super::ast::stm>{
        let return_token = self.cur_token.clone().unwrap().clone();
        self.next_token();
        //TODO: Skipping expressions
        while self.cur_token_is(super::lex::TokenType::SEMICOLON){
            self.next_token();
        }
        Some(super::ast::stm::Ret_Stm(super::ast::ret_stm_node{
            token: return_token,
            ret_value: super::ast::exp::Node("todo".to_owned()) 
        }))
    }
    fn parse_stm(&mut self) -> Option<super::ast::stm>{
        
        let tok = self.cur_token.clone().unwrap_or(super::lex::Token{
            kind: super::lex::TokenType::ILLEGAL,
            literal: "".to_owned()
        });
       match tok.kind{
           super::lex::TokenType::LET => self.parse_let_stm(),
           super::lex::TokenType::RETURN => self.parse_return_stm(),
           _ =>  None 
       } 
    }
    fn peek_error(&mut self, t: super::lex::TokenType)  {
       let  msg = format!("Expected next token to be {:#?}, got {:#?} instead",t, self.peek_token.to_owned().unwrap().kind); 
       self.errors.push(msg.to_owned());
    }
    pub fn parse_program(&mut self) -> Option<super::ast::Program>{
        let mut program = super::ast::Program  {
            statements: vec![],
            errors: vec![]
        };
        while  !self.cur_token_is(super::lex::TokenType::EOF){
                let stm = self.parse_stm();
                if stm != None {
                program.statements.push(stm.unwrap()); 
                }
                self.next_token();
        }
        Some(program)
    }
}

fn main(){

}