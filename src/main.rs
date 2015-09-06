use std::char;

mod roll {
    pub fn capitalize(word: &str) -> String {
        let capitalized = word.char_indices()
            .flat_map(|(i, c)| if i == 0 { 
                char::to_uppercase(c).next() 
            } else { Some(c) })
            .collect::<String>();

        println!("{}", capitalized);
        capitalized
    }

    pub fn decapitalize(word: &str) -> String {
        let decapitalized = word.char_indices()
            .flat_map(|(i, c)| if i == 0 { 
                char::to_lowercase(c).next() 
            } else { Some(c) })
            .collect::<String>();

        println!("{}", decapitalized);
        decapitalized
    }

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
    fn test_capitalize_fn() {
        let input = "hello world";
        let output = "Hello world";
        assert!(roll::capitalize(&input) == output);
    }

    #[test]
    fn test_decapitalize_fn() {
        let input = "HELLO WORLD";
        let output = "hELLO WORLD";
        assert!(roll::decapitalize(&input) == output);
    }

    #[test]
    fn test_uppercase_fn() {
        let input = "hello world";
        let output = "HELLO WORLD";
        assert!(roll::uppercase(&input) == output);
    }
}
