#[path = "../lexer/lex.rs"] mod lex;
#[path = "../ast/ast.rs"] mod ast;
mod parser;

#[cfg(test)]
mod tests {
    
    #[test]
    fn simple_test() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";
        struct test_iden{
            expected_identifier: String,
        }
        let expected_idens = vec![
            test_iden{
                expected_identifier: "x".to_string()
            },
            test_iden{
                expected_identifier: "y".to_string()
            },
            test_iden {
                expected_identifier: "foobar".to_string()
            },
        ];
        let l: super::lex::Lexer= super::lex::lexer_of_str(input);
        let mut p = super::parser::cons_parser(l );
        let program_stms = p.parse_program(); 
        for i in 0..std::vec::Vec::len(&program_stms.unwrap()){

        }
    }
   
}
