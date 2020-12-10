#[path = "../ast/ast.rs"]
mod ast;
#[path = "../lexer/lex.rs"]
mod lex;
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
        struct test_iden {
            expected_identifier: String,
        }
        let expected_idens = vec![
            test_iden {
                expected_identifier: "x".to_string(),
            },
            test_iden {
                expected_identifier: "y".to_string(),
            },
            test_iden {
                expected_identifier: "foobar".to_string(),
            },
        ];
        let l: super::lex::Lexer = super::lex::lexer_of_str(input);
        let mut p = super::parser::cons_parser(l);
        let program = p.parse_program().unwrap();
        check_parser_errors(&program);
        let program_stms = program.statements.clone();
        for i in 0..std::vec::Vec::len(&program_stms) {
            let stm = program_stms[i].clone();
            if !test_let_stm(stm, expected_idens[i].expected_identifier.clone()) {
                panic!("called to test_let_stm returned false");
            }
        }
    }

    #[test]
    fn test_return_stm() {
        let input = "
        return 5;
        return 10;
        return 993322;";

        let l: super::lex::Lexer = super::lex::lexer_of_str(input);
        let mut p = super::parser::cons_parser(l);
        let program = p.parse_program().unwrap();
        check_parser_errors(&program);
        if std::vec::Vec::len(&program.statements) != 3 {
            panic!(
                "Program.statements does not have 3 statements:  {:#?}",
                std::vec::Vec::len(&program.statements)
            )
        }

        for i in 0..std::vec::Vec::len(&program.statements) {
            if super::ast::Program::token_literal_of_stm(&program.statements[i]) != "return" {
                panic!(
                    "program.statement at {} is not 'return', got token literal {}",
                    i,
                    super::ast::Program::token_literal_of_stm(&program.statements[i])
                )
            }
        }
    }

    #[test]
    fn test_iden_exp() {
        let input = "foobar";

        let l: super::lex::Lexer = super::lex::lexer_of_str(input);
        let mut p = super::parser::cons_parser(l);
        let program = p.parse_program().unwrap();
        check_parser_errors(&program);
        
        assert_eq!(program.statements.len(),1);
        let exp_stm = match &program.statements[0] {
            super::ast::stm::Exp_Stm(e) => e,
            _ => panic!("None expstm!")
        };

        assert_eq!(exp_stm.token.literal,"foobar");
    }
    fn check_parser_errors(program: &super::ast::Program) {
        let errs = &program.errors;
        if std::vec::Vec::len(&errs) == 0 {
            return;
        }
        println!("Parser errors: {:#?}", std::vec::Vec::len(&errs));
        for i in 0..std::vec::Vec::len(&errs) {
            println!("Parser Error: {} ", errs[i]);
        }
        panic!()
    }

    fn test_let_stm(stm: super::ast::stm, name: String) -> bool {
        let first_tok = super::ast::Program::token_literal_of_stm(&stm);
        let let_stm = match &stm {
            super::ast::stm::Let_Stm(s) => Some(s),
            _ => None,
        };

        if first_tok != "let" {
            panic!("stmn.token_literal()) not 'let'.  Was: {}", first_tok);
        }
        if *let_stm.unwrap().name.value != name {
            panic!("let_stm.name.value is not equal to name");
        }
        if let_stm.unwrap().name.clone().token_literal() != name {
            panic!("let_stm.name.literal (actual token field) is not equal to name");
        }
        return true;
    }
}
