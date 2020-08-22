#[path = "../lexer/lex.rs"] mod lex;
#[path = "../ast/ast.rs"] mod ast;
mod parser;

#[cfg(test)]
mod tests {
    
    #[test]
    fn three_assignments() {
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
        let program_stms = p.parse_program().unwrap().statements; 
        for i in 0..std::vec::Vec::len(&program_stms){
            let stm = program_stms[i].clone();
            if !test_let_stm(stm,expected_idens[i].expected_identifier.clone()){
                panic!("called to test_let_stm returned false");
            }
        }
    }

    fn test_let_stm(stm:super::ast::stm,name: String) -> bool{
        let let_stm = match stm {
            super::ast::stm::Let_Stm(s) => s,
            super::ast::stm::Stm(s) => s
        };
        if let_stm.token_literal() != "let"{
            panic!("stmn.token_literal()) not 'let'. ");
        }
        if *let_stm.name.value != name {
            panic!("let_stm.name.value is not equal to name");
        }
        if let_stm.name.clone().token_literal() != name{
            panic!("let_stm.name.literal (actual token field) is not equal to name");
        }
        return true;
    }
}