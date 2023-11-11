use std::collections::HashSet;
use std::fmt::{self, Debug};

use rand::{seq::SliceRandom, thread_rng};

use crate::dictionary::Dictionary;

pub trait Beehive {
    fn gen_empty() -> Self;
    fn get_words(&self) -> Vec<String>;
    fn next_pattern(&self) -> Option<String>;

    fn push_word(&mut self, word: String);

    fn has_duplicates(&self) -> bool {
        let iter = self.get_words().clone().into_iter();
        let words_set: HashSet<String> = HashSet::from_iter(iter);

        words_set.len() < self.get_words().len()
    }
    fn get_char(&self, word_index: usize, char_index: usize) -> char {
        let words = self.get_words();

        words[word_index].chars().nth(char_index).unwrap()
    }
}

pub fn _recursive_generate<B: Beehive + Clone>(
    dictionary: &Dictionary,
    beehive: B,
    depth: i32,
) -> Option<B> {
    // beehive invalid
    if beehive.has_duplicates() {
        return None;
    }

    let next_pattern = beehive.next_pattern();

    if let Some(pattern) = next_pattern {
        let common_candidates = &dictionary
            .find_common_candidates(pattern.clone())
            .unwrap_or(vec![]);
        let mut common_candidates = common_candidates.clone();
        common_candidates.shuffle(&mut thread_rng());

        let mut candidates = common_candidates;

        if depth > 8 {
            let all_candidates = &dictionary._find_all_candidates(pattern).unwrap_or(vec![]);
            let mut all_candidates = all_candidates.clone();
            all_candidates.shuffle(&mut thread_rng());

            candidates.append(&mut all_candidates);
        }

        let mut candidates = candidates.into_iter();

        while let Some(candidate) = candidates.next() {
            let mut incr_beehive = beehive.clone();
            incr_beehive.push_word(candidate);
            if let Some(solved_waffle) = _recursive_generate(dictionary, incr_beehive, depth + 1) {
                return Some(solved_waffle);
            }
        }
    } else {
        // beehive full
        return Some(beehive);
    }

    // no behive found
    None
}

#[derive(Clone, Debug)]
pub struct SmallestBeehive {
    // size 1 cell
    words: Vec<String>,
}
impl Beehive for SmallestBeehive {
    fn gen_empty() -> Self {
        Self { words: vec![] }
    }
    fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }
    fn push_word(&mut self, word: String) {
        self.words.push(word);
    }

    fn next_pattern(&self) -> Option<String> {
        let pattern = match &self.words.len() {
            0 => Some("..".to_string()),
            1 => Some(format!("{}.", &self.get_char(0, 0))),
            2 => Some(format!("{}.", &self.get_char(0, 1))),
            3 => Some(format!("{}.", &self.get_char(1, 1))),
            4 => Some(format!("{}.", &self.get_char(2, 1))),
            5 => Some(format!("{}{}", &self.get_char(3, 1), &self.get_char(4, 1))),
            _ => None,
        };

        pattern
    }
}
impl fmt::Display for SmallestBeehive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            | {} {}\n\
            |{}   {}\n\
            | {} {}",
            self.get_char(2, 0),
            self.get_char(2, 1),
            self.get_char(0, 0),
            self.get_char(4, 1),
            self.get_char(3, 0),
            self.get_char(3, 1),
        )
    }
}

#[derive(Clone, Debug)]
struct SmallBeehive {
    // size 2 cells
    words: Vec<String>,
}
impl Beehive for SmallBeehive {
    fn gen_empty() -> Self {
        Self { words: vec![] }
    }
    fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }
    fn push_word(&mut self, word: String) {
        self.words.push(word);
    }

    fn next_pattern(&self) -> Option<String> {
        let pattern = match &self.words.len() {
            0 => Some("....".to_string()),
            1 => Some(format!(".{}", &self.words[0].chars().nth(0).unwrap())),
            2 => Some(format!("{}.", &self.words[1].chars().nth(0).unwrap())),
            3 => Some(format!("{}...", &self.words[2].chars().nth(1).unwrap())),
            4 => Some(format!("{}.", &self.words[3].chars().nth(3).unwrap())),
            5 => Some(format!(
                "{}{}",
                &self.words[0].chars().nth(3).unwrap(),
                &self.words[4].chars().nth(1).unwrap()
            )),
            6 => Some(format!(
                "{}.{}",
                &self.words[3].chars().nth(1).unwrap(),
                &self.words[0].chars().nth(2).unwrap()
            )),
            7 => Some(format!(
                "{}{}{}",
                &self.words[0].chars().nth(1).unwrap(),
                &self.words[6].chars().nth(1).unwrap(),
                &self.words[3].chars().nth(2).unwrap()
            )),
            _ => None,
        };

        pattern
    }
}
impl fmt::Display for SmallBeehive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            | {} {} {} {}\n\
            |{}   {}   {}\n\
            | {} {} {} {}",
            self.words[0].chars().nth(0).unwrap(),
            self.words[0].chars().nth(1).unwrap(),
            self.words[0].chars().nth(2).unwrap(),
            self.words[0].chars().nth(3).unwrap(),
            self.words[1].chars().nth(0).unwrap(),
            self.words[6].chars().nth(1).unwrap(),
            self.words[4].chars().nth(1).unwrap(),
            self.words[3].chars().nth(0).unwrap(),
            self.words[3].chars().nth(1).unwrap(),
            self.words[3].chars().nth(2).unwrap(),
            self.words[3].chars().nth(3).unwrap(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct MediumBeehive {
    // size 3 cells
    words: Vec<String>,
}
impl Beehive for MediumBeehive {
    fn gen_empty() -> Self {
        Self { words: vec![] }
    }
    fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }
    fn push_word(&mut self, word: String) {
        self.words.push(word);
    }

    fn next_pattern(&self) -> Option<String> {
        let pattern = match &self.words.len() {
            0 => Some("....".to_string()),
            1 => Some(format!(".{}", &self.words[0].chars().nth(0).unwrap())),
            2 => Some(format!("{}...", &self.words[1].chars().nth(0).unwrap())),
            3 => Some(format!("{}.", &self.words[2].chars().nth(3).unwrap())),
            4 => Some(format!("{}...", &self.words[3].chars().nth(1).unwrap())),
            5 => Some(format!(
                "{}{}",
                &self.words[0].chars().nth(3).unwrap(),
                &self.words[4].chars().nth(3).unwrap()
            )),
            6 => Some(format!(
                "{}..{}",
                &self.words[2].chars().nth(1).unwrap(),
                &self.words[4].chars().nth(2).unwrap()
            )),
            7 => Some(format!(
                "{}{}.{}",
                &self.words[2].chars().nth(2).unwrap(),
                &self.words[6].chars().nth(1).unwrap(),
                &self.words[0].chars().nth(2).unwrap()
            )),
            8 => Some(format!(
                "{}{}{}{}",
                &self.words[0].chars().nth(1).unwrap(),
                &self.words[7].chars().nth(2).unwrap(),
                &self.words[6].chars().nth(2).unwrap(),
                &self.words[4].chars().nth(1).unwrap()
            )),
            _ => None,
        };

        pattern
    }
}
impl fmt::Display for MediumBeehive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            | {} {} {} {}\n\
            |{}   {}   {}\n\
            | {} {} {} {}\n\
            |  {}   {}\n\
            |   {} {}",
            self.words[0].chars().nth(0).unwrap(),
            self.words[0].chars().nth(1).unwrap(),
            self.words[0].chars().nth(2).unwrap(),
            self.words[0].chars().nth(3).unwrap(),
            self.words[1].chars().nth(0).unwrap(),
            self.words[8].chars().nth(1).unwrap(),
            self.words[4].chars().nth(3).unwrap(),
            self.words[6].chars().nth(0).unwrap(),
            self.words[6].chars().nth(1).unwrap(),
            self.words[6].chars().nth(2).unwrap(),
            self.words[6].chars().nth(3).unwrap(),
            self.words[2].chars().nth(2).unwrap(),
            self.words[4].chars().nth(1).unwrap(),
            self.words[3].chars().nth(0).unwrap(),
            self.words[3].chars().nth(1).unwrap(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct BigBeehive {
    // size 4 cells
    words: Vec<String>,
}
impl Beehive for BigBeehive {
    fn gen_empty() -> Self {
        Self { words: vec![] }
    }
    fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }
    fn push_word(&mut self, word: String) {
        self.words.push(word);
    }

    fn next_pattern(&self) -> Option<String> {
        let pattern = match &self.words.len() {
            0 => Some("....".to_string()),
            1 => Some(format!(".{}", &self.get_char(0, 0))),
            2 => Some(format!("{}...", &self.get_char(1, 0))),

            3 => Some(format!("{}....", &self.get_char(2, 1))),
            4 => Some(format!(
                "{}{}.{}",
                &self.get_char(2, 2),
                &self.get_char(3, 1),
                &self.get_char(0, 2)
            )),
            5 => Some(format!(
                "{}{}{}..",
                &self.get_char(0, 1),
                &self.get_char(4, 2),
                &self.get_char(3, 2)
            )),

            6 => Some(format!(
                "{}.{}.",
                &self.get_char(2, 3),
                &self.get_char(5, 4)
            )),
            7 => Some(format!(
                "{}{}{}.",
                &self.get_char(6, 1),
                &self.get_char(5, 3),
                &self.get_char(3, 3)
            )),
            8 => Some(format!(
                "{}{}{}.",
                &self.get_char(0, 3),
                &self.get_char(7, 3),
                &self.get_char(3, 4)
            )),

            9 => Some(format!("{}{}", &self.get_char(6, 3), &self.get_char(8, 3))),

            _ => None,
        };

        pattern
    }
}
impl fmt::Display for BigBeehive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            | {} {} {} {}\n\
            |{}   {}   {}\n\
            | {} {} {} {} {}\n\
            |  {}   {}   {}\n\
            |   {} {} {} {}",
            self.get_char(0, 0),
            self.get_char(0, 1),
            self.get_char(0, 2),
            self.get_char(0, 3),
            self.get_char(1, 0),
            self.get_char(4, 2),
            self.get_char(7, 3),
            self.get_char(3, 0),
            self.get_char(3, 1),
            self.get_char(3, 2),
            self.get_char(3, 3),
            self.get_char(3, 4),
            self.get_char(4, 0),
            self.get_char(7, 1),
            self.get_char(9, 1),
            self.get_char(6, 0),
            self.get_char(6, 1),
            self.get_char(6, 2),
            self.get_char(6, 3),
        )
    }
}

#[cfg(test)]
mod test {}
