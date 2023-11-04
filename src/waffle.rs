use std::{result::Result, collections::HashSet};
use std::fmt;

use rand::{thread_rng, seq::SliceRandom};

use crate::dictionary::Dictionary;

struct Waffle {
    top_word: String,
    bottom_word: String,
    left_word: String,
    right_word: String,
    horizontal_word: String,
    vertical_word: String,
}

impl Waffle {
    pub fn generate(dictionary: Dictionary) -> Result<Self, ()> {
        let top_candidates = &dictionary.find_candidates(".....".to_string())?;
        let mut top_candidates = top_candidates.clone();

        top_candidates.shuffle(&mut thread_rng());
        let mut top_candidates = top_candidates.into_iter();

        while let Some(top_candidate) = top_candidates.next() {
            // left word
            let left_word_pattern = format!("{}....", &top_candidate.chars().nth(0).unwrap());
            let left_candidates = &dictionary.find_candidates(left_word_pattern.to_string())?;
            let mut left_candidates = left_candidates.clone();
            left_candidates.shuffle(&mut thread_rng());
            let mut left_candidates = left_candidates.into_iter();

            while let Some(left_candidate) = left_candidates.next() {
                //  horizontal word
                let horizontal_word_pattern = format!("{}....", &left_candidate.chars().nth(2).unwrap());
                let horizontal_candidates = &dictionary.find_candidates(horizontal_word_pattern.to_string())?;
                let mut horizontal_candidates = horizontal_candidates.clone();
                horizontal_candidates.shuffle(&mut thread_rng());
                let mut horizontal_candidates = horizontal_candidates.into_iter();

                while let Some(horizontal_candidate) = horizontal_candidates.next() {
                    //  vertical word
                    let vertical_word_pattern = format!("{}.{}..", &top_candidate.chars().nth(2).unwrap(), &horizontal_candidate.chars().nth(2).unwrap());
                    let vertical_candidates = &dictionary.find_candidates(vertical_word_pattern.to_string())?;
                    let mut vertical_candidates = vertical_candidates.clone();
                    vertical_candidates.shuffle(&mut thread_rng());
                    let mut vertical_candidates = vertical_candidates.into_iter();

                    while let Some(vertical_candidate) = vertical_candidates.next() {
                        //  bottom word
                        let bottom_word_pattern = format!("{}.{}..", &left_candidate.chars().nth(4).unwrap(), &vertical_candidate.chars().nth(4).unwrap());
                        let bottom_candidates = &dictionary.find_candidates(bottom_word_pattern.to_string())?;
                        let mut bottom_candidates = bottom_candidates.clone();
                        bottom_candidates.shuffle(&mut thread_rng());
                        let mut bottom_candidates = bottom_candidates.into_iter();

                        while let Some(bottom_candidate) = bottom_candidates.next() {
                            // right word
                            let right_word_pattern = format!("{}.{}.{}", &top_candidate.chars().nth(4).unwrap(), &horizontal_candidate.chars().nth(4).unwrap(), &bottom_candidate.chars().nth(4).unwrap());
                            let right_candidates = &dictionary.find_candidates(right_word_pattern.to_string())?;
                            let mut right_candidates = right_candidates.clone();
                            right_candidates.shuffle(&mut thread_rng());
                            let mut right_candidates = right_candidates.into_iter();

                            if let Some(right_candidate) = right_candidates.next() {

                                let waffle = Self {
                                    top_word: top_candidate,
                                    left_word: left_candidate,
                                    bottom_word: bottom_candidate,
                                    right_word: right_candidate,
                                    horizontal_word: horizontal_candidate,
                                    vertical_word: vertical_candidate,
                                };

                                return Ok(waffle);
                            }
                        }
                    }
                }
            }
        }

        Err(())
    }

}

#[derive(Debug, Clone)]
struct Waffle2 {
    words: Vec<String>,
}
impl Waffle2 {
    pub fn is_complete(&self) -> bool {
        self.words.len() == 8usize
    }

    pub fn has_duplicates(&self) -> bool {
        let iter = self.words.clone().into_iter();
        let words_set: HashSet<String>  = HashSet::from_iter(iter);

        words_set.len() < self.words.len()
    }

    pub fn generate_pattern(&self) -> String {
        let pattern = match &self.words.len() {
            0 =>         ".......".to_string(),
            1 => format!("{}......",    &self.words[0].chars().nth(0).unwrap()),
            2 => format!("{}......",    &self.words[1].chars().nth(2).unwrap()),
            3 => format!("{}.{}....",   &self.words[0].chars().nth(2).unwrap(), &self.words[2].chars().nth(2).unwrap()),
            4 => format!("{}.{}....",   &self.words[1].chars().nth(4).unwrap(), &self.words[3].chars().nth(4).unwrap()),
            5 => format!("{}.{}.{}..",  &self.words[0].chars().nth(4).unwrap(), &self.words[2].chars().nth(4).unwrap(), &self.words[4].chars().nth(4).unwrap()),
            6 => format!("{}.{}.{}..",  &self.words[1].chars().nth(6).unwrap(), &self.words[3].chars().nth(6).unwrap(), &self.words[5].chars().nth(6).unwrap()),
            7 => format!("{}.{}.{}.{}", &self.words[0].chars().nth(6).unwrap(), &self.words[2].chars().nth(6).unwrap(), &self.words[4].chars().nth(6).unwrap(), &self.words[6].chars().nth(6).unwrap()),
            _ =>         ".......".to_string(),
        };

        pattern
    }

    pub fn recursive_generate(dictionary: &Dictionary, waffle: Waffle2) -> Option<Waffle2> {
        // exit conditions
        if waffle.has_duplicates() {
            println!("disqualified waffle for duplicate word {:?}", &waffle);
            return None;
        }
        if waffle.is_complete() {
            return Some(waffle)
        }

        let pattern = waffle.generate_pattern();
        let candidates = &dictionary.find_candidates(pattern).unwrap_or(vec![]);
        let mut candidates = candidates.clone();

        candidates.shuffle(&mut thread_rng());
        let mut candidates = candidates.into_iter();

        while let Some(candidate) = candidates.next() {
            let mut incr_waffle = waffle.clone();
            incr_waffle.words.push(candidate);
            if let Some(solved_waffle) = Waffle2::recursive_generate(dictionary, incr_waffle) {
                return Some(solved_waffle);
            }
        };

        None
    }

    pub fn generate(dictionary: &Dictionary) -> Option<Waffle2> {
        let empty_waffle = Waffle2 {
            words: vec![]
        };

        Waffle2::recursive_generate(dictionary, empty_waffle)
    }
}
impl fmt::Display for Waffle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\
        {} {} {}\n\
        {}\n\
        {} {} {}\n\
        {}",
        self.top_word,
        self.left_word.chars().nth(1).unwrap(), self.vertical_word.chars().nth(1).unwrap(), self.right_word.chars().nth(1).unwrap(),
        self.horizontal_word,
        self.left_word.chars().nth(3).unwrap(), self.vertical_word.chars().nth(3).unwrap(), self.right_word.chars().nth(3).unwrap(),
        self.bottom_word)
    }
}
impl fmt::Display for Waffle2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\
            {} {} {} {}\n\
            {}\n\
            {} {} {} {}\n\
            {}\n\
            {} {} {} {}\n\
            {}",
            self.words[0],
            self.words[1].chars().nth(1).unwrap(), self.words[3].chars().nth(1).unwrap(), self.words[5].chars().nth(1).unwrap(), self.words[7].chars().nth(1).unwrap(),
            self.words[2],
            self.words[1].chars().nth(3).unwrap(), self.words[3].chars().nth(3).unwrap(), self.words[5].chars().nth(3).unwrap(), self.words[7].chars().nth(3).unwrap(),
            self.words[4],
            self.words[1].chars().nth(5).unwrap(), self.words[3].chars().nth(5).unwrap(), self.words[5].chars().nth(5).unwrap(), self.words[7].chars().nth(5).unwrap(),
            self.words[6],
        )
    }
}
#[cfg(test)]
mod test {
    use std::result::Result;
    use crate::dictionary::Dictionary;

    use super::{Waffle, Waffle2};

    #[test]
    fn gen_waffle() -> Result<(), ()> {
        let dictionary = Dictionary::new().map_err(|_e| ())?;

        let waffle = Waffle::generate(dictionary)?;

        println!("{}", waffle);

        Ok(())
    }

    #[test]
    fn gen_waffle2() -> Result<(), ()> {
        let dictionary = Dictionary::new().map_err(|_e| ())?;

        let res = Waffle2::generate(&dictionary);

        match res {
            Some(waffle) => {
                println!("{:?}", waffle);
                println!("{}", waffle);
            },
            None => println!("no waffle found")
        };

        Ok(())
    }

    
}