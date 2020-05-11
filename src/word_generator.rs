use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct WordGenerator {
    words: Vec<String>,
}

impl WordGenerator {
    pub fn from_file(path: String) -> Self {
        let file = std::fs::read_to_string(path).expect("File reading failed");
        let words: Vec<String> = file.lines()
            .map(|s| s.to_string())
            .collect();

        WordGenerator { words }
    }

    pub fn random_word(self) -> String {
        let mut rng = thread_rng();
        self.words.choose(&mut rng).expect("Failed to return word.").to_owned()
    }

    pub fn word_shuffle(word: &String) -> String {
        let mut rng = thread_rng();
        let mut chars: Vec<char> = word.chars().map(|c| c).collect();
        chars.shuffle(&mut rng);
        chars.into_iter().collect()
    }

    pub fn answer_options(word: &String) -> ([char; 4], usize) {
        let mut rng = thread_rng();
        let first_char = word.chars().next().unwrap();
        let mut chars: Vec<char> = word.chars().map(|c| c).collect();

        // Remove all duplicate characters
        chars.dedup();
        // Remove the first character
        chars = chars
            .iter()
            .filter_map(|c| if c != &first_char { Some(c.clone()) } else { None })
            .collect();

        // Create answer options
        let mut choosen_options: Vec<char> = chars.choose_multiple(&mut rng, 3).map(|c| c.clone()).collect();
        choosen_options.push(first_char);
        choosen_options.shuffle(&mut rng);
        assert_eq!(choosen_options.len(), 4);

        let correct_answer = choosen_options
            .iter()
            .position(|c| c == &first_char)
            .expect("");
        ([choosen_options[0], choosen_options[1], choosen_options[2], choosen_options[3]], correct_answer)
    }
}