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
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOTEQ
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
        (String::from("true"),TokenType::TRUE),
        (String::from("false"),TokenType::FALSE),
        (String::from("if"),TokenType::IF),
        (String::from("else"),TokenType::ELSE),
        (String::from("return"),TokenType::RETURN),
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

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace(){
            self.read_char();
        }
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
    
    fn read_num(&mut self) -> &str{
        let st = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        &self.input[st..self.position]
    }

    fn peek_char(&mut self) -> char{
        if self.readPosition >= String::len(&self.input){
            '\0'
        } else {
            self.input.to_string().as_bytes()[self.readPosition] as char
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
            let mut tok;
            match self.ch {
            '=' => {if self.peek_char() == '=' 
            {
                self.read_char();
                tok = cons_token(TokenType::EQ, "==".to_string())
            } else{
                tok = cons_token(TokenType::ASSIGN, self.ch.to_string())
                };
            },
            '\u{003b}' => tok = cons_token(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => tok = cons_token(TokenType::LPAREN, self.ch.to_string()),
            ')' => tok = cons_token(TokenType::RPAREN, self.ch.to_string()),
            ',' => tok = cons_token(TokenType::COMMA, self.ch.to_string()),
            '+' => tok = cons_token(TokenType::PLUS, self.ch.to_string()),
            '{' => tok = cons_token(TokenType::LBRACE, self.ch.to_string()),
            '}' => tok = cons_token(TokenType::RBRACE, self.ch.to_string()),
            '-' => tok = cons_token(TokenType::MINUS,  self.ch.to_string()),
            '!' =>  { 
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = cons_token(TokenType::NOTEQ, "!=".to_string())
                }
                else{
                tok = cons_token(TokenType::BANG,  self.ch.to_string())
                };
            },
            '*' => tok = cons_token(TokenType::ASTERISK,  self.ch.to_string()),
            '/' => tok = cons_token(TokenType::SLASH, self.ch.to_string()),
            '<' => tok = cons_token(TokenType::LT, self.ch.to_string()),
            '>' => tok = cons_token(TokenType::GT, self.ch.to_string()),
            '\0' => tok = cons_token(TokenType::EOF, '\0'.to_string()),
            chr => 
                if chr.is_alphabetic() || chr == '_' {
                    let iden = self.read_identifier();
                    let kind = keywords().get(&iden.to_string()).unwrap_or(&TokenType::IDENT).clone();
                    return cons_token(kind, iden.to_string())
                } else if chr.is_numeric(){
                    return cons_token(TokenType::INT,self.read_num().to_string())
                } else {
                    tok = cons_token(TokenType::ILLEGAL,chr.to_string())
                }
        };
        self.read_char();
        tok
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
    loop {
        tokens.push(lexer.next_token());
        if tokens.last().unwrap().kind == TokenType::EOF {break;}
    }
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
        Token{
            kind: TokenType::RPAREN,
            literal: String::from(")")
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
        Token{
            kind:TokenType::RBRACE,
            literal: String::from("}")
        },
        Token{
            kind: TokenType::SEMICOLON,
            literal:  String::from(";")
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
    println!("REPL for the Monkey Language! ");
    loop {
    let in_line = dialoguer::Input::<String>::new().with_prompt(">> ").interact().unwrap();

    let toks = lex(&in_line);
    println!("Given line, loop new iter: {}", in_line);
    println!("Tokenization : {:#?}" , toks);
    }

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
    
    #[test]
    fn test_2(){
     let input = "let five = 5;
     let ten = 10;

     let add = fn(x, y) {
         x + y;
     };
     let result = add(five, ten);
     ~-/*5;

     5 < 10 > 5;
     ";

    //assert_eq!(super::test_case_three_vec(),super::lex(input));
   

    }

}
