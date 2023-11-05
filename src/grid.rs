use simple_matrix::Matrix;
use std::{fmt, collections::HashSet};
use rand::{thread_rng, seq::SliceRandom};

use crate::dictionary::Dictionary;

#[derive(Clone, Debug)]
pub struct Grid {
    layout: Matrix<char>,
}

pub enum Kind {
    Row,
    Col,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            layout: Matrix::new(rows, cols),
        }
    }
    pub fn set(&mut self, row: usize, col: usize, val: char) {
        self.layout.set(row, col, val);
    }
    pub fn set_word(&mut self, i: usize, kind: &Kind, val: String) {
        match kind {
            Kind::Row => self.set_row_word(i, val),
            Kind::Col => self.set_col_word(i, val),
        }
    }
    pub fn set_row_word(&mut self, row: usize, val: String) {
        for (col, char) in val.chars().enumerate() {
            self.set(row, col, char);
        };
    }
    pub fn set_col_word(&mut self, col: usize, val: String) {
        for (row, char) in val.chars().enumerate() {
            self.set(row, col, char);
        };
    }

    pub fn get_row(&self, row: usize) -> Option<String> {
        self.layout.get_row(row).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_col(&self, col: usize) -> Option<String> {
        self.layout.get_col(col).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&char> {
        self.layout.get(row, col)
    }
    pub fn next_pattern(&self, i: usize, kind: &Kind) -> Option<String> {
        let res = match kind {
            Kind::Row => self.get_row(i),
            Kind::Col => self.get_col(i),
        }.map(|s| s.replace('\0', "."));

        res
    }
    pub fn is_invalid(&self, dictionary: &Dictionary) -> bool {
        self.has_duplicates()
        || self.has_isolated_letters()
        || self.has_forbidden_tupples(dictionary)
        || self.has_isles()
    }

    pub fn has_duplicates(&self) -> bool {
        let mut words_set: HashSet<String>  = HashSet::new();

        for i in 0..self.layout.rows() {
            let words = self.get_row(i).unwrap_or(String::from("\0"));
            let words: Vec<String> = words.split('_').map(|w| String::from(w)).collect();
            let words: Vec<String> = words
                .into_iter()
                .filter(|w| w.len() > 1)
                .filter(|w| !w.contains('\0'))
                .collect();

            for word in words {
                if words_set.contains(&word) {
                    return true;
                }
                words_set.insert(word);
            };
        };

        for j in 0..self.layout.cols() {
            let words = self.get_col(j).unwrap_or(String::from("\0"));
            let words: Vec<String> = words.split('_').map(|w| String::from(w)).collect();
            let words: Vec<String> = words
                .into_iter()
                .filter(|w| w.len() > 1)
                .filter(|w| !w.contains('\0'))
                .collect();

            for word in words {
                if words_set.contains(&word) {
                    return true;
                }
                words_set.insert(word);
            };
        };

        false
    }

    pub fn has_isolated_letters(&self) -> bool {
        for r in 0..self.layout.rows() {
            for c in 0..self.layout.cols() {
                let center = self.get_cell(r, c).unwrap_or(&'_');
                let top = if r == 0 { &'_' } else { self.get_cell(r-1, c).unwrap_or(&'_') };
                let bottom = self.get_cell(r+1, c).unwrap_or(&'_');
                let left = if c == 0 { &'_' } else { self.get_cell(r, c-1).unwrap_or(&'_') };
                let right = self.get_cell(r, c+1).unwrap_or(&'_');

                let neighbours = format!("{}{}{}{}", top, bottom, left, right);

                if center != &'_' && neighbours == "____".to_string() {
                    return true;
                }
            }
        }

        false
    }
    pub fn has_forbidden_tupples(&self, dictionary: &Dictionary) -> bool {
        for r in 0..self.layout.rows() {
            let row = self.get_row(r).unwrap_or("".to_string());
            if dictionary.has_forbidden_tuples(row).unwrap_or(false) {
                return true;
            }
        }
        for c in 0..self.layout.cols() {
            let col = self.get_col(c).unwrap_or("".to_string());
            if dictionary.has_forbidden_tuples(col).unwrap_or(false) {
                return true;
            }
        }

        false
    }

    pub fn has_isles(&self) -> bool {
        // let mut i = 1;
        // let asd: Matrix<i32> = Matrix::new(self.layout.rows(), self.layout.cols());

        // let top_left = self.get_cell(0,0).unwrap_or('\0');
        false
    }

    fn _get_depth(&self) -> usize {
        for c in 0..self.layout.cols() {
            for o in 0..1 {
                if self.get_cell(c + o, c).unwrap_or(&'\0') == &'\0' {
                    return 2*c + o;
                }
            }
        }
        0
    }

    pub fn recursive_generate(&self, dictionary: &Dictionary, i: usize, kind: Kind) -> Option<Self> {
        // grid invalid
        if self.is_invalid(dictionary) {
            return None;
        }
    
        let next_pattern = self.next_pattern(i, &kind);
    
        if let Some(pattern) = next_pattern {
    
            let common_candidates = &dictionary.find_candidates_allow_split(pattern.clone()).unwrap_or(vec![]);
            let mut common_candidates = common_candidates.clone();
            common_candidates.shuffle(&mut thread_rng());
    
            let candidates = common_candidates;
    
            // if depth > 8 {
            //     let all_candidates = &dictionary.find_all_candidates(pattern).unwrap_or(vec![]);
            //     let mut all_candidates = all_candidates.clone();
            //     all_candidates.shuffle(&mut thread_rng());
        
            //     candidates.append(&mut all_candidates);
            // }
    
            let mut candidates = candidates.into_iter();

    
            while let Some(candidate) = candidates.next() {
                let mut incr_grid = self.clone();
                incr_grid.set_word(i, &kind, candidate);

                let (incr_i, incr_kind) = match &kind {
                    Kind::Row => (i, Kind::Col),
                    Kind::Col => (i+1, Kind::Row),
                };
                if let Some(complete_grid) = incr_grid.recursive_generate(dictionary, incr_i, incr_kind) {
                    return Some(complete_grid);
                }
            };
        } else {
            // grid full
            return Some(self.clone());
        }
    
        // no grid found
        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let border: String = (0..self.layout.cols()+2).map(|_i| '─').collect();
        write!(f, "grid of size {}x{}\n", self.layout.rows(), self.layout.cols())?;
        write!(f, "┌{}┐\n", &border)?;
        for i in 0..self.layout.rows() {
            write!(f,
                "│ {} │\n",
                &self.get_row(i)
                    .unwrap_or(String::from(""))
                    .replace('\0', ".")
                    .replace('_', "▓")
                    .as_str()
                    .to_uppercase()
                )?;
        };
        write!(f, "└{}┘", &border)

    }
}
#[cfg(test)]
mod test {

    use std::time::Instant;

    use crate::dictionary;

    use super::{Grid, Kind};

    #[test]
    fn gen_grid() {
        let start = Instant::now();
        let dictionary = dictionary::Dictionary::new().unwrap();
        let elapsed = start.elapsed();
        println!("dictionary created in {:?}", elapsed);

        let grid = Grid::new(5,5);
        let full = grid.recursive_generate(&dictionary, 0, Kind::Row);
        let elapsed = start.elapsed();
        println!("grid created in {:?}", elapsed);
        println!("{:?}", full);

        if let Some(g) = full {
            println!("{}", g);
        }
        // grid.set(0, 0, '◼');
        // grid.set(0, 1, 'b');
        // grid.set(0, 2, 'o');
        // grid.set(1, 0, 'n');
        // grid.set(1, 1, 'j');
        // grid.set(1, 2, 'o');
        // grid.set(2, 0, 'u');
        // grid.set(2, 1, 'r');
        // grid.set(2, 2, '◼');


        // let pattern = &grid.next_pattern(0, Kind::Row);
        // println!("{:?}", pattern);

        // let pattern = &grid.next_pattern(0, Kind::Col);
        // println!("{:?}", pattern);

        // let pattern = &grid.next_pattern(3, Kind::Col);
        // println!("{:?}", pattern);
    }

    #[test]
    fn invalid_grid() {
        let mut grid = Grid::new(2,2);
        grid.set(0,0, '_');
        grid.set(0,1, 'a');
        grid.set(1,0, '_');
        grid.set(1,1, '_');

        let has_isolated = grid.has_isolated_letters();

        println!("{}", has_isolated);
    }
}