use simple_matrix::Matrix;
use std::{fmt, collections::HashSet};
use rand::{thread_rng, seq::SliceRandom};

use crate::dictionary::Dictionary;

#[derive(Clone, Debug)]
pub struct Grid {
    layout: Matrix<char>,
    resolved_lines: Vec<Line>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Row,
    Col,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    index: usize,
    kind: Kind,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            layout: Matrix::new(rows, cols),
            resolved_lines: vec![],
        }
    }
    
    pub fn rows(&self) -> usize {
        self.layout.rows()
    }
    pub fn cols(&self) -> usize {
        self.layout.cols()
    }

    // getters
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
    pub fn get_line(&self, line: &Line) -> Option<String> {
        match line.kind {
            Kind::Row => self.get_row(line.index),
            Kind::Col => self.get_col(line.index),
        }
    }
    
    // setters
    pub fn set_cell(&mut self, row: usize, col: usize, val: char) {
        self.layout.set(row, col, val);
    }
    pub fn set_line(&mut self, line: &Line, val: String) {
        let i = line.index;
        match line.kind {
            Kind::Row => self.set_row(i, val),
            Kind::Col => self.set_col(i, val),
        }
    }
    pub fn set_row(&mut self, row: usize, val: String) {
        for (col, char) in val.chars().enumerate() {
            self.set_cell(row, col, char);
        };
    }
    pub fn set_col(&mut self, col: usize, val: String) {
        for (row, char) in val.chars().enumerate() {
            self.set_cell(row, col, char);
        };
    }

    pub fn flag_resolved(&mut self, line: &Line) {
        self.resolved_lines.push(line.clone());
    }
    pub fn next_line(&self) -> Option<Line> {
        let rows: Vec<Line> = (0..self.rows())
            .map(|i| Line { index: i, kind: Kind::Row })
            .collect();
        let cols: Vec<Line> = (0..self.cols())
            .map(|i| Line { index: i, kind: Kind::Col })
            .collect();
        let lines = [rows.as_slice(), cols.as_slice()].concat();

        let mut unresolved_lines: Vec<Line> = lines
            .into_iter()
            .filter(|l| !self.resolved_lines.contains(l))
            .collect();
        
        unresolved_lines.sort_by(|line_a, line_b| {
            let val_a = self.get_line(line_a).unwrap().replace('\0', ".");
            let val_b = self.get_line(line_b).unwrap().replace('\0', ".");

            // count the number of constrained cells, cells with already a letter or a '_' and not a '.'
            let liberties_a = &val_a.chars().filter(|c| c == &'.').count();
            let liberties_b = &val_b.chars().filter(|c| c == &'.').count();

            let blacks_a = &val_a.chars().filter(|c| c == &'_').count();
            let blacks_b = &val_b.chars().filter(|c| c == &'_').count();

            let constrains_a = &val_a.len() - liberties_a - blacks_a;
            let constrains_b = &val_b.len() - liberties_b - blacks_b;

            // println!("line_a {:?}-{}, val '{}', {} liberties, {} blacks, {} constrains", &line_a.kind, &line_a.index, &val_a.replace('\0', "."), liberties_a, blacks_a, constrains_a);
            // println!("line_a {:?}-{}, val '{}', {} liberties, {} blacks, {} constrains", &line_b.kind, &line_b.index, &val_b.replace('\0', "."), liberties_b, blacks_b, constrains_b);

            // most constrained first
            match constrains_b.cmp(&constrains_a) {
                std::cmp::Ordering::Equal => liberties_b.cmp(liberties_a),
                res => res
            }
        });

        unresolved_lines.into_iter().next()
    }

    // incr
    pub fn pattern(&self, line: &Line) -> Option<String> {
        self
            .get_line(line)
            .map(|s| s.replace('\0', "."))
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

    pub fn recursive_generate(&self, dictionary: &Dictionary) -> Option<Self> {
        let opt_next_line = self.next_line();
        // grid is complete
        if opt_next_line.is_none() {
            return Some(self.clone());
        }

        // grid invalid
        if self.is_invalid(dictionary) {
            return None;
        }

        let next_line = opt_next_line.unwrap();

        let next_pattern = self.pattern(&next_line);

        if let Some(pattern) = next_pattern {
    
            // strict layout
            // let common_candidates = &dictionary.recursive_find_candidates(pattern.clone()).unwrap_or(vec![]);
            // allow the algo to add blanks
            let common_candidates = &dictionary.find_candidates_allow_split(pattern.clone()).unwrap_or(vec![]);
            let mut common_candidates = common_candidates.clone();
            common_candidates.shuffle(&mut thread_rng());
    
            let candidates = common_candidates;
            let mut candidates = candidates.into_iter();

            while let Some(candidate) = candidates.next() {
                let mut incr_grid = self.clone();
                incr_grid.set_line(&next_line, candidate);
                incr_grid.flag_resolved(&next_line);

                if let Some(complete_grid) = incr_grid.recursive_generate(dictionary) {
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

        let grid = Grid::new(3, 2);
        let full = grid.recursive_generate(&dictionary);
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
        grid.set_cell(0,0, '_');
        grid.set_cell(0,1, 'a');
        grid.set_cell(1,0, '_');
        grid.set_cell(1,1, '_');

        let has_isolated = grid.has_isolated_letters();

        println!("{}", has_isolated);
    }
}