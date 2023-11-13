use core::fmt;

use rand::{thread_rng, seq::SliceRandom};
use simple_matrix::Matrix;

use crate::grid_beehive::GridBeehive;

#[derive(Debug, Clone)]
pub struct BeehiveSwap {
    solved_layout: Matrix<char>,
    shuffled_layout: Matrix<char>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Row,
    Col,
    Diag,
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    row: usize,
    col: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    index: usize,
    kind: Kind,
}

impl From<GridBeehive> for BeehiveSwap {
    fn from(value: GridBeehive) -> Self {
        let rows = value.rows();
        let cols = value.cols();
        let mut solved_layout = Matrix::new(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                solved_layout.set(r, c, value.get_cell(r, c).unwrap().clone());
            }
        }

        let shuffled_layout = solved_layout.clone();

        let mut swap = Self {
            solved_layout,
            shuffled_layout,
        };

        swap.shuffle(10);

        swap
    }
}
impl BeehiveSwap {
    pub fn rows(&self) -> usize {
        self.solved_layout.rows()
    }
    pub fn cols(&self) -> usize {
        self.solved_layout.cols()
    }
    pub fn diags(&self) -> usize {
        self.solved_layout.rows() + self.solved_layout.cols() - 1
    }
    pub fn get_solved_line(&self, line: &Line) -> Option<String> {
        match line.kind {
            Kind::Row => self.get_solved_row(line.index),
            Kind::Col => self.get_solved_col(line.index),
            Kind::Diag => self.get_solved_diag(line.index),
        }
    }
    pub fn get_solved_row(&self, row: usize) -> Option<String> {
        self.solved_layout.get_row(row).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_solved_col(&self, col: usize) -> Option<String> {
        self.solved_layout.get_col(col).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_solved_diag(&self, diag: usize) -> Option<String> {
        if diag >= self.rows() + self.cols() - 1 {
            return None;
        }

        let res: String = (0..(diag + 1))
            .map(|i| self.get_solved_cell(&Cell { row: diag - i, col: i}))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        Some(res)
    }
    pub fn get_solved_cell(&self, cell: &Cell) -> Option<&char> {
        self.solved_layout.get(cell.row, cell.col)
    }
    pub fn get_shuffled_cell(&self, cell: &Cell) -> Option<&char> {
        self.shuffled_layout.get(cell.row, cell.col)
    }
    pub fn get_cells(&self) -> Vec<Cell> {
        let mut cells = vec![];
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                cells.push(Cell{row: r, col: c});
            }
        }

        cells
    }
    pub fn get_swappable_cells(&self) -> Vec<Cell> {
        self.get_cells()
            .into_iter()
            .filter(|c| self.get_solved_cell(c) != Some(&'_') )
            .filter(|c| c != &Cell { row: 0, col: 2 })
            .filter(|c| c != &Cell { row: 2, col: 2 })
            .filter(|c| c != &Cell { row: 2, col: 3 })
            .filter(|c| c != &Cell { row: 2, col: 5 })
            .filter(|c| c != &Cell { row: 3, col: 2 })
            .filter(|c| c != &Cell { row: 5, col: 0 })
            .collect()
    }

    pub fn set_shuffled_cell(&mut self, cell: &Cell, val: char) {
        self.shuffled_layout.set(cell.row, cell.col, val);
    }


    pub fn shuffle(&mut self, swaps_cnt: usize) {
        let swappable_cells = self.get_swappable_cells();
        for _s in 0..swaps_cnt {
            let mut swap = swappable_cells.clone();
            swap.shuffle(&mut thread_rng());
            swap = swap[0..2].to_vec();

            let cell_a = &swap[0];
            let cell_b = &swap[1];

            println!("swapping {:?} with {:?}", cell_a, cell_b);

            let char_a = *(&self.get_shuffled_cell(cell_a).unwrap().clone());
            let char_b = *(&self.get_shuffled_cell(cell_b).unwrap().clone());

            self.set_shuffled_cell(cell_a, char_b);
            self.set_shuffled_cell(cell_b, char_a);
        }
    }
}

impl fmt::Display for BeehiveSwap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "beehive-swap of size {}x{}\n",
            self.rows(),
            self.cols()
        )?;
        write!(f, "Solved layout\n")?;
        for i in 0..self.rows() {
            let offset: String = (0..i).map(|_i| ' ').collect();
            write!(f, "{}", offset)?;
            for j in 0..self.cols() {
                write!(
                    f,
                    "{} ",
                    self.get_solved_cell(&Cell{ row: i, col: j })
                        .unwrap_or(&'\0')
                        .to_string()
                        .replace('\0', "⬡")
                        .replace('_', "⬢")
                        .as_str()
                        .to_uppercase()
                )?;
            }
            write!(f, "\n")?;
        }

        write!(f, "Shuffled layout\n")?;
        for i in 0..self.rows() {
            let offset: String = (0..i).map(|_i| ' ').collect();
            write!(f, "{}", offset)?;
            for j in 0..self.cols() {
                write!(
                    f,
                    "{} ",
                    self.get_shuffled_cell(&Cell{ row: i, col: j })
                        .unwrap_or(&'\0')
                        .to_string()
                        .replace('\0', "⬡")
                        .replace('_', "⬢")
                        .as_str()
                        .to_uppercase()
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

pub mod ui {
    use leptos::*;
    // use stylers::style;
    use super::*;
    #[component]
    pub fn BeehiveSwapComponent(beehive: BeehiveSwap) -> impl IntoView {
        // let (value, _set_value) = create_signal(beehive);

        let rows = *(&beehive.rows());
        let cols = *(&beehive.cols());

        let cell_views = (0..beehive.rows())
        .map(move |r| {
            let offsets_before = (0..r).map(|_| view! { <div class="offset" /> }).collect_view();
            let cells = (0..beehive.cols())
                .map(|c| {
                    let solved_letter = beehive.get_solved_cell(&Cell{row: r, col: c}).unwrap().clone();
                    let shuffled_letter = beehive.get_shuffled_cell(&Cell{row: r, col: c}).unwrap().clone();
                    match shuffled_letter {
                        '_' => view! { <div class="empty-cell" /> },
                        '\0' => view! { <div class="cell" /> },
                        letter if letter == solved_letter => view! {
                            <div
                                class="cell is-correct"
                            >
                                "" {solved_letter.to_uppercase().to_string()} ""
                            </div>
                        },
                        letter => view! {
                            <div
                                class="cell"
                            >
                                "" {letter.to_uppercase().to_string()} ""
                            </div>
                        }
                    }
                }).collect_view();
            let offsets_after = (0..(beehive.rows()-r -1 )).map(|_| view! { <div class="offset" /> }).collect_view();

            vec![offsets_before, cells, offsets_after].into_iter().collect_view()
        }).collect_view();

        view! {
            <div
                class="beehive-container"
                style:width = move || format!("{}em", 2* cols + rows - 1)
                style:grid-template-columns = move || format!("repeat({}, minmax(0, 1fr))", 2* cols + rows - 1)
            >
                { cell_views }
            </div>
        }
    }
}


#[test]
fn test_shuffle() {
    let mut grid = GridBeehive::new(6,6);
    grid.set_row(0, "__yeah".to_string());
    grid.set_row(1, "_h__so".to_string());
    grid.set_row(2, "sofa_t".to_string());
    grid.set_row(3, "t_r_i_".to_string());
    grid.set_row(4, "a_ex__".to_string());
    grid.set_row(5, "the___".to_string());

    let beehive_swap = BeehiveSwap::from(grid);

    println!("{}", beehive_swap);
    // println!("{:?}", beehive_swap.get_swappable_cells().len());

}