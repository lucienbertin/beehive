use simple_matrix::Matrix;
use std::fmt;

#[derive(Debug)]
struct Grid {
    layout: Matrix<char>,
}

enum Kind {
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

    pub fn get_row(&self, row: usize) -> Option<String> {
        self.layout.get_row(row).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    // pub fn gen_pattern(self, x: usize, y: usize, kind: Kind) -> String {

    // }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "\
            | {} |\n\
            | {} |\n\
            | {} |",
            &self.get_row(0).unwrap_or(String::from("")),
            &self.get_row(1).unwrap_or(String::from("")),
            &self.get_row(2).unwrap_or(String::from("")),
        )
    }
}
#[cfg(test)]
mod test {
    use super::Grid;

    #[test]
    fn asasd() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, '◼');
        grid.set(0, 1, 'b');
        grid.set(0, 2, 'o');
        grid.set(1, 0, 'n');
        grid.set(1, 1, 'j');
        grid.set(1, 2, 'o');
        grid.set(2, 0, 'u');
        grid.set(2, 1, 'r');
        grid.set(2, 2, '◼');
        println!("{:?}", grid);

        println!("{}", grid);
    }
}