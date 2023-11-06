use rand::{seq::SliceRandom, thread_rng, Rng};
use simple_matrix::Matrix;
use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use crate::dictionary::Dictionary;

#[derive(Clone, Debug)]
pub struct GridBeehive {
    layout: Matrix<char>,
    resolved_lines: Vec<Line>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Row,
    Col,
    Diag,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    row: usize,
    col: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    index: usize,
    kind: Kind,
}

impl GridBeehive {
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
    pub fn diags(&self) -> usize {
        self.layout.rows() + self.layout.cols() - 1
    }

    // getters
    pub fn get_line(&self, line: &Line) -> Option<String> {
        match line.kind {
            Kind::Row => self.get_row(line.index),
            Kind::Col => self.get_col(line.index),
            Kind::Diag => self.get_diag(line.index),
        }
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
    pub fn get_diag(&self, diag: usize) -> Option<String> {
        if diag >= self.layout.rows() + self.layout.cols() - 1 {
            return None;
        }

        let res: String = (0..(diag + 1))
            .map(|i| self.get_cell(diag - i, i))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        Some(res)
    }
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&char> {
        self.layout.get(row, col)
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
            Kind::Diag => self.set_diag(i, val),
        }
    }
    pub fn set_row(&mut self, row: usize, val: String) {
        for (col, char) in val.chars().enumerate() {
            self.set_cell(row, col, char);
        }
    }
    pub fn set_col(&mut self, col: usize, val: String) {
        for (row, char) in val.chars().enumerate() {
            self.set_cell(row, col, char);
        }
    }
    pub fn set_diag(&mut self, diag: usize, val: String) {
        // convert usize to i32 for substracting below 0
        let i_diag: i32 = diag.try_into().unwrap();
        let i_rows: i32 = self.layout.rows().try_into().unwrap();
        let i_offset = std::cmp::max(i_diag - i_rows + 1, 0);
        let offset: usize = i_offset.try_into().unwrap();

        for (i, char) in val.chars().enumerate() {
            self.set_cell(diag - i - offset, i + offset, char);
        }
    }

    pub fn flag_resolved(&mut self, line: &Line) {
        self.resolved_lines.push(line.clone());
    }

    // incr
    pub fn pattern(&self, line: &Line) -> Option<String> {
        let res = match line.kind {
            Kind::Row => self.get_row(line.index),
            Kind::Col => self.get_col(line.index),
            Kind::Diag => self.get_diag(line.index),
        }
        .map(|s| s.replace('\0', "."));

        res
    }
    pub fn next_line(&self) -> Option<Line> {
        let rows: Vec<Line> = (0..self.rows())
            .map(|i| Line {
                index: i,
                kind: Kind::Row,
            })
            .collect();
        let cols: Vec<Line> = (0..self.cols())
            .map(|i| Line {
                index: i,
                kind: Kind::Col,
            })
            .collect();
        let diags: Vec<Line> = (0..self.diags())
            .map(|i| Line {
                index: i,
                kind: Kind::Diag,
            })
            .collect();

        let lines = [rows.as_slice(), cols.as_slice(), diags.as_slice()].concat();

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
                res => res,
            }
        });

        unresolved_lines.into_iter().next()
    }

    // invalid
    pub fn is_invalid(&self, dictionary: &Dictionary) -> bool {
        self.has_duplicates() || self.has_forbidden_tupples(dictionary) || self.has_isles()
    }
    fn has_duplicates(&self) -> bool {
        let mut words_set: HashSet<String> = HashSet::new();

        for r in 0..self.rows() {
            let words = self.get_row(r).unwrap_or(String::from("\0"));
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
            }
        }

        for c in 0..self.cols() {
            let words = self.get_col(c).unwrap_or(String::from("\0"));
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
            }
        }

        for d in 0..self.diags() {
            let words = self.get_col(d).unwrap_or(String::from("\0"));
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
            }
        }

        false
    }
    fn has_forbidden_tupples(&self, dictionary: &Dictionary) -> bool {
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
        let diag_cnt = self.layout.rows() + self.layout.cols() - 1;
        for d in 0..diag_cnt {
            let diag = self.get_diag(d).unwrap_or("".to_string());
            if dictionary.has_forbidden_tuples(diag).unwrap_or(false) {
                return true;
            }
        }

        false
    }
    pub fn has_isles(&self) -> bool {
        let mut visited_cells: HashSet<Cell> = HashSet::new();
        // visit all blacks to enforce boundaries
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if self.get_cell(r, c).is_some() && self.get_cell(r, c).unwrap() == &'_' {
                    visited_cells.insert(Cell { row: r, col: c });
                }
            }
        }

        // find an anchor
        let mut rng = rand::thread_rng();
        let mut anchor = Cell { row: 0, col: 0 };
        while self.get_cell(anchor.row, anchor.col).unwrap_or(&'_') == &'_' {
            anchor = Cell {
                row: rng.gen_range(0..self.rows()),
                col: rng.gen_range(0..self.cols()),
            };
        }
        let mut visit_queue = VecDeque::new();
        visit_queue.push_front(anchor);

        while let Some(cell) = visit_queue.pop_back() {
            // visit cell
            visited_cells.insert(cell.clone());

            // find neighbours
            let mut neighbours = vec![];
            if cell.row > 0 {
                let top = Cell {
                    row: cell.row - 1,
                    col: cell.col,
                };
                neighbours.push(top);
                let top_right = Cell {
                    row: cell.row - 1,
                    col: cell.col + 1,
                };
                neighbours.push(top_right);
            }
            if cell.col > 0 {
                let left = Cell {
                    row: cell.row,
                    col: cell.col - 1,
                };
                neighbours.push(left);
                let bottom_left = Cell {
                    row: cell.row + 1,
                    col: cell.col - 1,
                };
                neighbours.push(bottom_left);
            }
            let bottom = Cell {
                row: cell.row + 1,
                col: cell.col,
            };
            neighbours.push(bottom);
            let right = Cell {
                row: cell.row,
                col: cell.col + 1,
            };
            neighbours.push(right);

            // add unvisited neighbours to the visit_queue
            neighbours
                .into_iter()
                .filter(|n| self.get_cell(n.row, n.col).is_some()) // only keep cells that are in the grid
                .filter(|n| !visited_cells.contains(&n)) // and that aren't visited yet
                .for_each(|n| visit_queue.push_front(n));
        }

        visited_cells.len() < self.rows() * self.cols()
    }

    fn _get_depth(&self) -> usize {
        for c in 0..self.layout.cols() {
            for o in 0..1 {
                if self.get_cell(c + o, c).unwrap_or(&'\0') == &'\0' {
                    return 2 * c + o;
                }
            }
        }
        0
    }

    pub fn recursive_generate(&self, dictionary: &Dictionary) -> Option<Self> {
        println!("{}", self);

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
        // println!("{:?}", next_pattern);

        if let Some(pattern) = next_pattern {
            // strict layout
            // let common_candidates = &dictionary.recursive_find_candidates(pattern.clone()).unwrap_or(vec![]);
            // allow the algo to add blanks
            let common_candidates = &dictionary
                .find_candidates_allow_split(pattern.clone())
                .unwrap_or(vec![]);
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
            }
        } else {
            // grid full
            return Some(self.clone());
        }

        // no grid found
        None
    }
}

impl fmt::Display for GridBeehive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let border: String = (0..(2*self.layout.cols())+1).map(|_i| '─').collect();
        write!(
            f,
            "grid of size {}x{}\n",
            self.layout.rows(),
            self.layout.cols()
        )?;
        // write!(f, " {} \n", &border)?;
        for i in 0..self.layout.rows() {
            let offset: String = (0..i).map(|_i| ' ').collect();
            write!(f, "{}", offset)?;
            for j in 0..self.layout.cols() {
                write!(
                    f,
                    "{} ",
                    self.get_cell(i, j)
                        .unwrap_or(&'\0')
                        .to_string()
                        .replace('\0', "⬡")
                        .replace('_', "⬢")
                        .as_str()
                        .to_uppercase()
                )?;
            }
            write!(f, "\n")?;
            // write!(f,
            //     "│ {} │\n",
            //     &self.get_row(i)
            //         .unwrap_or(String::from(""))
            //         .replace('\0', ".")
            //         .replace('_', "⬢")
            //         .as_str()
            //         .to_uppercase()
            //     )?;
        }
        // let offset: String = (0..self.layout.rows()).map(|_i| ' ').collect();

        // write!(f, " {} {} ", offset, &border)
        write!(f, "")
    }
}
#[cfg(test)]
mod test {

    use std::time::Instant;

    use crate::{dictionary, grid_beehive::Line};

    use super::{GridBeehive, Kind};

    // #[test]
    // fn gen_grid_beehive() {
    //     let start = Instant::now();
    //     let dictionary = dictionary::Dictionary::new().unwrap();
    //     let elapsed = start.elapsed();
    //     println!("dictionary created in {:?}", elapsed);

    //     let mut empty = GridBeehive::new(5,5);
    //     empty.set_cell(0, 0, '_');
    //     // empty.set(0, 3, '_');
    //     empty.set_cell(1, 1, '_');
    //     empty.set_cell(1, 3, '_');
    //     // empty.set(3, 0, '_');
    //     empty.set_cell(3, 1, '_');
    //     empty.set_cell(3,3, '_');
    //     empty.set_cell(4,4, '_');

    //     println!("{}", empty);

    //     let full = empty.recursive_generate(&dictionary, 0, Kind::Row);
    //     let elapsed = start.elapsed();
    //     println!("grid created in {:?}", elapsed);
    //     println!("{:?}", full);

    //     if let Some(g) = full {
    //         println!("{}", g);
    //     }
    // }
    // #[test]
    // fn gen_grid_beehive_2() {
    //     let start = Instant::now();
    //     let dictionary = dictionary::Dictionary::new().unwrap();
    //     let elapsed = start.elapsed();
    //     println!("dictionary created in {:?}", elapsed);

    //     let mut empty = GridBeehive::new(5,5);
    //     empty.set_row(0, "_tied".to_string());
    //     empty.set_row(1, "a_r_o".to_string());
    //     empty.set_row(2, "root_".to_string());
    //     empty.set_row(3, "t_n__".to_string());
    //     empty.set_row(4, "so___".to_string());

    //     println!("{}", empty);

    //     let full = empty.recursive_generate(&dictionary, 0, Kind::Row);
    //     let elapsed = start.elapsed();
    //     println!("grid created in {:?}", elapsed);
    //     println!("{:?}", full);

    //     if let Some(g) = full {
    //         println!("{}", g);
    //     }
    // }
    #[test]
    fn gen_grid_beehive_3() {
        let start = Instant::now();
        let dictionary = dictionary::Dictionary::new().unwrap();
        let elapsed = start.elapsed();
        println!("dictionary created in {:?}", elapsed);

        let mut empty = GridBeehive::new(4, 5);
        // empty.set_row(0, "__..._".to_string());
        // empty.set_row(1, "_._...".to_string());
        // empty.set_row(2, "._._..".to_string());
        // empty.set_row(3, ".._._.".to_string());
        // empty.set_row(4, "..._._".to_string());
        // empty.set_row(5, "_...__".to_string());

        println!("initial layout{}", empty);

        let full = empty.recursive_generate(&dictionary);
        let elapsed = start.elapsed();
        println!("grid created in {:?}", elapsed);
        println!("{:?}", full);

        if let Some(g) = full {
            println!("{}", g);
        }
    }

    #[test]
    fn test_next_line() {
        let start = Instant::now();
        let dictionary = dictionary::Dictionary::new().unwrap();
        let elapsed = start.elapsed();
        println!("dictionary created in {:?}", elapsed);

        let mut empty = GridBeehive::new(2, 2);
        let line = Line {
            index: 1,
            kind: Kind::Diag,
        };
        empty.set_line(&line, "a_".to_string());
        empty.flag_resolved(&line);
        println!("{}", empty);

        let line = empty.next_line();
        println!("{:?}", line);
    }
}
