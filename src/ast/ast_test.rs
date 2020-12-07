#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let sample_input = String::from("=+(){},;");
        assert_eq!(super::test_case_one_vec(), super::lex::lex("=+(){},;"))
    }

}
