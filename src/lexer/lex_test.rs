mod lex;
use lex::Token;
use lex::TokenType;

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
#[cfg(test)]
mod tests {
    
    #[test]
    fn simple_test() {
        let sample_input = String::from("=+(){},;");
        assert_eq!(super::test_case_one_vec(), super::lex::lex("=+(){},;"))
    }
   #[test] 
    fn test_1() {
     let input = "let five = 5;
     let ten = 10;
     let add = fn(x, y) {
    x + y; 
    };
    
    let result = add(five,ten); ";

    assert_eq!(super::test_case_two_vec(), super::lex::lex(input));
   
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
