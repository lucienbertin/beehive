use std::{fs::File, result::Result, io::{BufReader, Read}};

pub struct Dictionary {
    pub common_words: Vec<String>,
    pub all_words: Vec<String>,
}
// pub enum Language {
//     English,
//     French,
// }
impl Dictionary {
    fn parse_file(path: &str) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(path)?;

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let mut split = contents.split('\n');
        split.next(); // remove header line
    
        let lines = split.collect::<Vec<&str>>();
    
        let words: Vec<String> = lines
            .iter()
            .map(|&line| String::from(line)).collect();

        Ok(words)
    }
    pub fn new() -> Result<Self, std::io::Error> {
        // let path = match language {
        //     Language::English => format!("dictionaries/{}/all_words", "english"),
        //     Language::French => format!("dictionaries/{}/all_words", "french"),
        // };

        // let path = "dictionaries/english/most_frequent_words";
        // let path = "dictionaries/english/wordle_list";

        let common_words = Dictionary::parse_file("dictionaries/english/most_frequent_words")?;
        let all_words = Dictionary::parse_file("dictionaries/english/all_words")?;
        let dictionary = Self {
            common_words,
            all_words,
        };

        Ok(dictionary)
    }

    pub fn find_common_candidates(&self, pattern: String) -> Result<Vec<String>, ()> {
        let candidates = self.common_words
            .clone()
            .into_iter()
            .filter(|w| match_pattern(w, &pattern))
            .map(|w| String::from(w))
            .collect();

        Ok(candidates)
    }
    pub fn find_all_candidates(&self, pattern: String) -> Result<Vec<String>, ()> {
        let candidates = self.all_words
            .clone()
            .into_iter()
            .filter(|w| match_pattern(w, &pattern))
            .map(|w| String::from(w))
            .collect();

        Ok(candidates)
    }
    
}

fn match_pattern(word: &String, pattern: &String) -> bool {
    if word.len() != pattern.len() {
        return false;
    };

    let mut result = true;

    let mut word_chars = word.chars();
    let mut pattern_chars = pattern.chars();
    while let Some(pattern_char) = pattern_chars.next() {
        let word_char = word_chars.next().unwrap();
        result &= pattern_char == '.' || pattern_char == word_char;
    };

    result
}

#[cfg(test)]
mod test {
    use std::result::Result;
    use rand::{self, Rng};
    use super::{Dictionary};

    #[test]
    fn init_english_dict() -> Result<(), ()> {
        let dict = Dictionary::new().map_err(|_e| ())?;

        let candidates = dict.find_common_candidates("a.w.e".to_string())?;

        for word in &candidates {
            println!("{}", word);
        }

        let mut rng = rand::thread_rng();
        let candidate_index = rng.gen_range(0..candidates.len());
        let candidate = &candidates[candidate_index];

        println!("{}", candidate);

        Ok(())
    }
}