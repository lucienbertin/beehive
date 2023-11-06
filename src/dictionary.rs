use std::{fs::File, result::Result, io::{BufReader, Read, Write}};

pub struct Dictionary {
    pub common_words: Vec<String>,
    pub all_words: Vec<String>,
    pub forbidden_tuples: Vec<String>,
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
    pub fn new() -> Result<Self, ()> {
        // let path = match language {
        //     Language::English => format!("dictionaries/{}/all_words", "english"),
        //     Language::French => format!("dictionaries/{}/all_words", "french"),
        // };

        // let path = "dictionaries/english/most_frequent_words";
        // let path = "dictionaries/english/wordle_list";

        let common_words = Dictionary::parse_file("dictionaries/english/most_frequent_words").map_err(|_e| ())?;
        let all_words = Dictionary::parse_file("dictionaries/english/all_words").map_err(|_e| ())?;
        let forbidden_tuples = Dictionary::parse_file("dictionaries/english/forbidden_tuples").map_err(|_e| ())?;
        let dictionary = Self {
            common_words,
            all_words,
            forbidden_tuples,
        };

        Ok(dictionary)
    }

    // fn gen_forbidden_tuples(&mut self) -> Result<(), ()>{
    //     let mut tuples: Vec<String> = vec![];
    //     for a in "abcdefghijklmnopqrstuvwxyz".chars() {
    //         for b in "abcdefghijklmnopqrstuvwxyz".chars() {
    //             tuples.push(vec![a,b].into_iter().collect());
    //         }
    //     }
    //     for a in "abcdefghijklmnopqrstuvwxyz".chars() {
    //         for b in "abcdefghijklmnopqrstuvwxyz".chars() {
    //             for c in "abcdefghijklmnopqrstuvwxyz".chars() {
    //                 tuples.push(vec![a,b,c].into_iter().collect());
    //             }
    //         }
    //     }
    //     // let size_2_tuples = vec!["wxyz".to_string(), "ad".to_string()];

    //     self.forbidden_tuples = tuples
    //         .into_iter()
    //         .filter(|t| {
    //             let exists = self.common_words
    //                 .clone()
    //                 .into_iter()
    //                 .any(|w| w.contains(t.as_str()));

    //             !exists
    //         }).collect();

    //     write_dictionary("dictionaries/english/forbidden_tuples", &self.forbidden_tuples);
    //     Ok(())
    // }

    pub fn find_common_candidates(&self, pattern: String) -> Result<Vec<String>, ()> {
        if pattern == ".".to_string() {
            return Ok(vec![".".to_string()]);
        }
        if pattern == "_".to_string() {
            return Ok(vec!["_".to_string()]);
        }
        let candidates = self.common_words
            .clone()
            .into_iter()
            .filter(|w| match_pattern(w, &pattern))
            .map(|w| String::from(w))
            .collect();

        Ok(candidates)
    }
    pub fn _find_all_candidates(&self, pattern: String) -> Result<Vec<String>, ()> {
        let candidates = self.all_words
            .clone()
            .into_iter()
            .filter(|w| match_pattern(w, &pattern))
            .map(|w| String::from(w))
            .collect();

        Ok(candidates)
    }

    pub fn recursive_find_candidates(&self, pattern: String) -> Result<Vec<String>, ()> {

        let res = if pattern == "_".to_string() {
            self.find_common_candidates(pattern)
        } else if pattern.contains('_') {
            let index = pattern.find('_').unwrap();
            if index == 0 {
                let sub_pattern = (&pattern.clone()).chars().into_iter().skip(1).collect();
                self.recursive_find_candidates(sub_pattern)
                    .map(|candidates| candidates.into_iter().map(|c| format!("_{}", c).to_string()).collect())

            } else if index == pattern.len() - 1 {
                let sub_pattern = String::from(&pattern[..index]);
                self.recursive_find_candidates(sub_pattern)
                    .map(|candidates| candidates.into_iter().map(|c| format!("{}_", c).to_string()).collect())

            } else {
                let left_pattern = String::from(&pattern[..index]);
                let right_pattern: String = (&pattern.clone()).chars().into_iter().skip(index+1).collect();
    
                let left_candidates = self.recursive_find_candidates(left_pattern)?;
                let right_candidates = self.recursive_find_candidates(right_pattern)?;
    
                if left_candidates.len() == 0 {
                    Ok(vec![])
                } else if right_candidates.len() == 0 {
                    Ok(vec![])
                } else {
                    let mut temp = vec![];
                    for l in &left_candidates {
                        for r in &right_candidates {
                            temp.push(format!("{}_{}", l, r));
                        }
                    }
    
                    Ok(temp)
                }
            }

        } else {
            self.find_common_candidates(pattern)
        };

        res
    }

    pub fn find_candidates_allow_split(&self, pattern: String) -> Result<Vec<String>, ()> {
        if pattern.contains('.') {
            let mut res = self.recursive_find_candidates(pattern.clone())?;

            for (i, _c) in (&pattern.clone()).chars().enumerate().filter(|(_i, c)| c == &'.') {
                let left = String::from(&pattern[..i]);
                let right = String::from(&pattern[i+1..]);
                let split_pattern = format!("{}_{}", left, right); // doable via replace_range it seems

                let mut candidates = self.recursive_find_candidates(split_pattern)?;
                res.append(&mut candidates);
            };

            Ok(res)
        } else {
            self.recursive_find_candidates(pattern)
        }
    }

    pub fn has_forbidden_tuples(&self, pattern: String) -> Result<bool, ()> {
        let mut any_forbidden_duo = false;
        if pattern.len() > 1 {
            any_forbidden_duo = (0..(&pattern.len()-1))
                .into_iter()
                .map(|i| String::from(&pattern.as_str()[i..i+2]))
                .filter(|d| !d.contains('\0'))
                .filter(|d| !d.contains('_'))
                .any(|d| self.forbidden_tuples.contains(&d));
        };
        let mut any_forbidden_trio = false;
        if pattern.len() > 2 {
            any_forbidden_trio = (0..(pattern.len()-2))
                .into_iter()
                .map(|i| String::from(&pattern.as_str()[i..i+3]))
                .filter(|d| !d.contains('\0'))
                .filter(|d| !d.contains('_'))
                .any(|d| self.forbidden_tuples.contains(&d));
        };
        Ok(any_forbidden_duo || any_forbidden_trio)
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

fn _write_dictionary(path: &str, words:&Vec<String>) -> Result<(), ()> {
    let mut file = File::create(path).map_err(|_e| ())?;
    for word in words {
        file.write_all(word.as_bytes()).map_err(|_e| ())?;
        file.write_all(b"\n").map_err(|_e| ())?;
    };

    Ok(())
}
#[cfg(test)]
mod test {
    use std::result::Result;
    use rand::{thread_rng, seq::SliceRandom};
    use super::{Dictionary};

    #[test]
    fn init_english_dict() -> Result<(), ()> {
        let dict = Dictionary::new().map_err(|_e| ())?;

        let pattern = ".ad.".to_string();

        let mut candidates = dict.find_candidates_allow_split(pattern.clone())?;
        candidates.shuffle(&mut thread_rng());

        println!("{} candidates match '{}'", candidates.len(), &pattern);

        for word in &candidates[..std::cmp::min(candidates.len(), 12)] {
            println!("{}", word);
        }


        // let mut rng = rand::thread_rng();
        // let candidate_index = rng.gen_range(0..candidates.len());
        // let candidate = &candidates[candidate_index];

        // println!("{}", candidate);

        Ok(())
    }
    #[test]
    fn test_forbidden_tuples() -> Result<(), ()> {
        let dict = Dictionary::new()?;

        println!("{} forbidden tuples", dict.forbidden_tuples.len());
        println!("{:?}", dict.forbidden_tuples);

        Ok(())
    }
}