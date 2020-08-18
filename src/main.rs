use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenType,
    literal: String,
}

#[derive(Debug, PartialEq)]
struct Lexer {
    input: String,
    position: usize,
    readPosition: usize,
    ch: char,
}

fn cons_token(token_kind: TokenType, lit: String) -> Token {
    Token {
        kind: token_kind,
        literal: lit,
    }
}

fn keywords() -> HashMap<String,TokenType>  {
    [
        (String::from("fn"), TokenType::FUNCTION),
        (String::from("let"), TokenType::LET),
    ]
    .iter()
    .cloned()
    .collect()
}
impl Lexer {

    fn read_identifier(&mut self ) -> &str{
        let st = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        &self.input[st..self.position]
    }

    fn read_char(&mut self) {
        self.ch = if self.readPosition >= self.input.len() {
            '\0'
        } else {
            self.input.to_string().as_bytes()[self.readPosition] as char
        };

        self.position = self.readPosition;
        self.readPosition += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => cons_token(TokenType::ASSIGN, self.ch.to_string()),
            ';' => cons_token(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => cons_token(TokenType::LPAREN, self.ch.to_string()),
            ')' => cons_token(TokenType::RPAREN, self.ch.to_string()),
            ',' => cons_token(TokenType::COMMA, self.ch.to_string()),
            '+' => cons_token(TokenType::PLUS, self.ch.to_string()),
            '{' => cons_token(TokenType::LBRACE, self.ch.to_string()),
            '}' => cons_token(TokenType::RBRACE, self.ch.to_string()),
            '\0' => cons_token(TokenType::EOF, '\0'.to_string()),
            chr => 
                if chr.is_alphabetic() || chr == '_' {
                    cons_token(keywords().get(&chr.to_string()).unwrap().clone(), String::from(self.read_identifier()))
                } else {
                    cons_token(TokenType::ILLEGAL,chr.to_string())
                }
        };
        self.read_char();
        return tok;
    }
}

fn lexer_of_str(string: &str) -> Lexer {
    let mut ret_lex: Lexer = Lexer {
        input: String::from(string),
        position: 0,
        readPosition: 0,
        ch: '0',
    };
    ret_lex.read_char();
    ret_lex
}

fn lex(sample_input: &str) -> std::vec::Vec<Token> {
    let mut lexer = lexer_of_str(sample_input);
    let mut tokens = Vec::new();
    for _ in 0..String::len(&lexer.input) {
        tokens.push(lexer.next_token());
    }
    tokens.push(lexer.next_token());
    tokens
}

fn test_case_one_vec() -> std::vec::Vec<Token> {
    vec![
        Token {
            kind: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            kind: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            kind: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            kind: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            kind: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            kind: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            kind: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            kind: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            kind: TokenType::EOF,
            literal: String::from("\0"),
        },
    ]
}

fn test_case_two_vec() -> std::vec::Vec<Token> {
    vec![
        Token {
            kind: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            kind: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            kind: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            kind: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            kind: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            kind: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            kind: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            kind: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            kind: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            kind: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            kind: TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        Token {
            kind: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            kind: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            kind: TokenType::RPAREN,
            literal: String::from("x"),
        },
        Token {
            kind: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            kind: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            kind: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            kind: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("result"),
        },
        Token {
            kind: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            kind: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            kind: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            kind: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            kind: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            kind: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            kind: TokenType::EOF,
            literal: String::from("\0"),
        },
    ]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let sample_input = String::from("=+(){},;");
        assert_eq!(super::test_case_one_vec(), super::lex("=+(){},;"))
    }
   #[test] 
    fn test_1() {
     let input = "let five = 5;
     let ten = 10;
     let add = fn(x, y) {
    x + y; 
    };
    
    let result = add(five,ten); ";

    assert_eq!(super::test_case_two_vec(),super::lex(input));
    }
}
