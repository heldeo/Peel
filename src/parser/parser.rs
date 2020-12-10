use std::collections::HashMap;
#[path = "../ast/ast.rs"]
mod ast;
#[path = "../lexer/lex.rs"]
mod lex;

pub enum PRECEDENCE{
    LOWEST = 1,
    EQUALS,
    LESSGREATER, // > or < 
    SUM,
    PRODUCT,
    PREFIX,
    CALL
}
pub struct parser {
    l: super::lex::Lexer,
    cur_token: Option<super::lex::Token>,
    peek_token: Option<super::lex::Token>,
    errors: Vec<String>,
    prefix_parse_fns: Option<HashMap<super::lex::TokenType, fn() -> super::ast::exp>>,
    infix_parse_fns: Option<HashMap<super::lex::TokenType, fn(super::ast::exp)>>,
}
fn something() -> super::ast::exp {
    super::ast::exp::Node("s".to_owned())
}
pub fn cons_parser(lexer: super::lex::Lexer) -> parser {
    let mut parser = parser {
        l: lexer,
        cur_token: None,
        peek_token: None,
        errors: vec![],
        prefix_parse_fns: None,
        infix_parse_fns: None,
    };
    parser.next_token();
    parser
}

impl parser {
    fn cur_token_is(&mut self, t: super::lex::TokenType) -> bool {
        if self.cur_token == None {
            false
        } else {
            t == self.cur_token.clone().unwrap().kind
        }
    }
    fn peek_token_is(&mut self, t: super::lex::TokenType) -> bool {
        t == self
            .peek_token
            .clone()
            .unwrap_or(super::lex::cons_eof_tok())
            .kind
    }
    pub fn peek_error(&mut self, t: super::lex::TokenType) {
        let msg = format!(
            "Expected next token to be {:#?}, got {:#?} instead",
            t,
            self.peek_token.to_owned().unwrap().kind
        );
        self.errors.push(msg.to_owned());
    }
    fn expect_peek(&mut self, t: super::lex::TokenType) -> bool {
        if self.peek_token_is(t.to_owned()) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token());
    }
    fn register_prefix(
        &mut self,
        t: super::lex::TokenType,
        prefix_parse_fn: fn() -> super::ast::exp,
    ) {
        self.prefix_parse_fns = Some(match &mut self.prefix_parse_fns {
            Some(m) => {
                m.insert(t, prefix_parse_fn);
                m.clone()
            }
            None => {
                let mut m = HashMap::new();
                m.insert(t, prefix_parse_fn);
                m
            }
        });
    }

    fn register_infx(&mut self, t: super::lex::TokenType, infix_parse_fn: fn(super::ast::exp)) {
        self.infix_parse_fns = Some(match &mut self.infix_parse_fns {
            Some(m) => {
                m.insert(t, infix_parse_fn);
                m.clone()
            }
            None => {
                let mut m = HashMap::new();
                m.insert(t, infix_parse_fn);
                m
            }
        });
    }

    fn parse_let_stm(&mut self) -> Option<super::ast::stm> {
        let cur_token = self.cur_token.clone().unwrap().clone();

        if !self.expect_peek(super::lex::TokenType::IDENT) {
            return None;
        }

        // current token here is identifier/
        //expect_peek sideffect of moving token head

        let node = super::ast::let_stm_node {
            token: cur_token,
            name: super::ast::iden {
                token: self.cur_token.clone().unwrap(),
                value: self.cur_token.clone().unwrap().literal,
            },
            value: super::ast::exp::Node("".to_string()),
        };

        if !self.expect_peek(super::lex::TokenType::ASSIGN) {
            return None;
        }
        while !self.cur_token_is(super::lex::TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(super::ast::stm::Let_Stm(node))
    }
    fn parse_return_stm(&mut self) -> Option<super::ast::stm> {
        let return_token = self.cur_token.clone().unwrap().clone();
        self.next_token();
        //TODO: Skipping expressions
        while !self.cur_token_is(super::lex::TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(super::ast::stm::Ret_Stm(super::ast::ret_stm_node {
            token: return_token,
            ret_value: super::ast::exp::Node("todo".to_owned()),
        }))
    }
    fn parse_exp_stm(&mut self) -> Option<super::ast::stm> {
        let tok_wrap = self.cur_token.as_ref();
        if (tok_wrap == None) {
            return None;
        };

        let tok = tok_wrap.unwrap().clone();

        let exp_stm = super::ast::stm::Exp_Stm(super::ast::exp_stm_node {
            token: tok,
            exp: self.parse_exp(PRECEDENCE::LOWEST).unwrap(),
        });
        if self.peek_token_is(super::lex::TokenType::SEMICOLON) {
            self.next_token()
        }
        Some(exp_stm)
    }
    fn parse_exp(&mut self, p: PRECEDENCE) -> Option<super::ast::exp> {
        super::ast::exp::Node(String::from(""));
        let pref = self.prefix_parse_fns.as_ref().unwrap().get(&self.cur_token.as_ref().unwrap().kind);

        if pref == None {None} else {Some(pref.unwrap()())}
    }
    fn parse_stm(&mut self) -> Option<super::ast::stm> {
        let tok = self.cur_token.clone().unwrap_or(super::lex::Token {
            kind: super::lex::TokenType::ILLEGAL,
            literal: "".to_owned(),
        });
        match tok.kind {
            super::lex::TokenType::LET => self.parse_let_stm(),
            super::lex::TokenType::RETURN => self.parse_return_stm(),
            _ => self.parse_exp_stm(),
        }
    }
    pub fn parse_program(&mut self) -> Option<super::ast::Program> {
        let mut program = super::ast::Program {
            statements: vec![],
            errors: vec![],
        };
        while !self.cur_token_is(super::lex::TokenType::EOF) {
            let stm = self.parse_stm();
            if stm != None {
                program.statements.push(stm.unwrap());
            }
            self.next_token();
        }
        Some(program)
    }
}

fn main() {}
