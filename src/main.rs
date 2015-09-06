use std::char;

mod roll {
    pub fn uppercase(word: &str) -> String {
        let uppercased = word.chars()
            .flat_map(char::to_uppercase)
            .collect::<String>();

        println!("{}", uppercased);
        uppercased
    }
}

mod tests {
    use roll;

    #[test]
    fn test_uppercase_fn() {
        let input = "hello_world";
        let output = "HELLO_WORLD";
        assert!(roll::uppercase(&input) == output);
    }
}
