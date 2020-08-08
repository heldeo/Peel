#[derive(Debug, PartialEq)]
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
    position: i32,
    readPosition: i32,
    ch: char,
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
            literal: String::from(""),
        },
    ]
}

fn lex(sample_input: &str) -> std::vec::Vec<Token> {
    let string_in = String::from(sample_input);
    vec![Token {
        kind: TokenType::EOF,
        literal: String::from(string_in),
    }]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let sample_input = String::from("=+(){},;");
        assert_eq!(super::test_case_one_vec(), super::lex("=+(){},;"));
    }
}
