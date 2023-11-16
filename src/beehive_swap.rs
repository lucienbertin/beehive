use core::fmt;

use rand::{seq::SliceRandom, thread_rng};
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
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Cell {
    row: usize,
    col: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    index: usize,
    kind: Kind,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Color {
    Green,
    Yellow,
    White,
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

        swap.shuffle(20);

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
    // getters solved layout
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
            .map(|i| {
                self.get_solved_cell(&Cell {
                    row: diag - i,
                    col: i,
                })
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        Some(res)
    }
    pub fn get_solved_cell(&self, cell: &Cell) -> Option<&char> {
        self.solved_layout.get(cell.row, cell.col)
    }

    // getters shuffled layout
    pub fn get_shuffled_line(&self, line: &Line) -> Option<String> {
        match line.kind {
            Kind::Row => self.get_shuffled_row(line.index),
            Kind::Col => self.get_shuffled_col(line.index),
            Kind::Diag => self.get_shuffled_diag(line.index),
        }
    }
    pub fn get_shuffled_row(&self, row: usize) -> Option<String> {
        self.shuffled_layout.get_row(row).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_shuffled_col(&self, col: usize) -> Option<String> {
        self.shuffled_layout.get_col(col).map(|iter| {
            let str: String = iter.collect();
            str
        })
    }
    pub fn get_shuffled_diag(&self, diag: usize) -> Option<String> {
        if diag >= self.rows() + self.cols() - 1 {
            return None;
        }

        let res: String = (0..(diag + 1))
            .map(|i| {
                self.get_shuffled_cell(&Cell {
                    row: diag - i,
                    col: i,
                })
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        Some(res)
    }
    pub fn get_shuffled_cell(&self, cell: &Cell) -> Option<&char> {
        self.shuffled_layout.get(cell.row, cell.col)
    }
    pub fn get_cells(&self) -> Vec<Cell> {
        let mut cells = vec![];
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                cells.push(Cell { row: r, col: c });
            }
        }

        cells
    }
    pub fn get_swappable_cells(&self) -> Vec<Cell> {
        self.get_cells()
            .into_iter()
            .filter(|c| self.get_solved_cell(c) != Some(&'_'))
            .filter(|c| c != &Cell { row: 0, col: 2 })
            .filter(|c| c != &Cell { row: 2, col: 2 })
            .filter(|c| c != &Cell { row: 2, col: 3 })
            .filter(|c| c != &Cell { row: 2, col: 5 })
            .filter(|c| c != &Cell { row: 3, col: 2 })
            .filter(|c| c != &Cell { row: 5, col: 0 })
            .collect()
    }

    fn _get_solved_words(&self, cell: &Cell) -> Vec<String> {
        // hardcode ftw
        match cell {
            Cell { row: 0, col: 2 } => vec![
                self.get_solved_row(0).unwrap()[2..6].to_string(),
                self.get_solved_diag(2).unwrap(),
            ],
            Cell { row: 0, col: 3 } => vec![self.get_solved_row(0).unwrap()[2..6].to_string()],
            Cell { row: 0, col: 4 } => vec![
                self.get_solved_row(0).unwrap()[2..6].to_string(),
                self.get_solved_col(4).unwrap()[0..2].to_string(),
            ],
            Cell { row: 0, col: 5 } => vec![
                self.get_solved_row(0).unwrap()[2..6].to_string(),
                self.get_solved_col(5).unwrap()[0..3].to_string(),
                self.get_solved_diag(5).unwrap()[2..6].to_string(),
            ],
            Cell { row: 1, col: 1 } => vec![self.get_solved_diag(2).unwrap()],
            Cell { row: 1, col: 4 } => vec![
                self.get_solved_row(1).unwrap()[4..6].to_string(),
                self.get_solved_diag(5).unwrap()[2..6].to_string(),
            ],
            Cell { row: 1, col: 5 } => vec![
                self.get_solved_row(1).unwrap()[4..6].to_string(),
                self.get_solved_col(5).unwrap()[0..3].to_string(),
            ],
            Cell { row: 2, col: 0 } => vec![
                self.get_solved_row(2).unwrap()[0..4].to_string(),
                self.get_solved_col(0).unwrap()[2..6].to_string(),
                self.get_solved_diag(2).unwrap(),
            ],
            Cell { row: 2, col: 1 } => vec![
                self.get_solved_row(2).unwrap()[0..4].to_string(),
                self.get_solved_row(1).unwrap()[1..3].to_string(),
                self.get_solved_diag(3).unwrap()[0..2].to_string(),
            ],
            Cell { row: 3, col: 0 } => vec![
                self.get_solved_col(0).unwrap()[2..6].to_string(),
                self.get_solved_diag(3).unwrap()[0..2].to_string(),
            ],
            Cell { row: 3, col: 4 } => vec![self.get_solved_diag(7).unwrap()],
            Cell { row: 4, col: 0 } => vec![self.get_solved_col(0).unwrap()[2..6].to_string()],
            Cell { row: 4, col: 2 } => vec![
                self.get_solved_row(4).unwrap()[2..4].to_string(),
                self.get_solved_col(2).unwrap()[2..6].to_string(),
                self.get_solved_diag(6).unwrap()[0..2].to_string(),
            ],
            Cell { row: 4, col: 3 } => vec![
                self.get_solved_row(4).unwrap()[2..4].to_string(),
                self.get_solved_diag(7).unwrap(),
            ],
            Cell { row: 5, col: 1 } => vec![
                self.get_solved_row(5).unwrap()[0..3].to_string(),
                self.get_solved_diag(6).unwrap()[0..2].to_string(),
            ],
            Cell { row: 5, col: 2 } => vec![
                self.get_solved_row(5).unwrap()[0..3].to_string(),
                self.get_solved_col(2).unwrap()[2..6].to_string(),
                self.get_solved_diag(7).unwrap(),
            ],
            _ => vec![],
        }
    }

    fn get_cell_color(&self, cell: &Cell) -> Color {
        if &self.get_solved_cell(cell) == &self.get_shuffled_cell(cell) {
            return Color::Green;
        };

        // hardcode ftw
        // code very easy to understand and maintain
        let is_yellow = match cell {
            Cell { row: 0, col: 3 } => {
                let r_word_solved = self.get_solved_row(0).unwrap()[2..6].to_string();
                let r_word_shuffled = self.get_shuffled_row(0).unwrap()[2..6].to_string();

                is_yellow(r_word_solved, r_word_shuffled, 1)
            }
            Cell { row: 0, col: 4 } => {
                let r_word_solved = self.get_solved_row(0).unwrap()[2..6].to_string();
                let r_word_shuffled = self.get_shuffled_row(0).unwrap()[2..6].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 2);

                let c_word_solved = self.get_solved_col(4).unwrap()[0..2].to_string();
                let c_word_shuffled = self.get_shuffled_col(4).unwrap()[0..2].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 0);

                r_yellow || c_yellow
            }
            Cell { row: 0, col: 5 } => {
                let r_word_solved = self.get_solved_row(0).unwrap()[2..6].to_string();
                let r_word_shuffled = self.get_shuffled_row(0).unwrap()[2..6].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 3);

                let c_word_solved = self.get_solved_col(5).unwrap()[0..3].to_string();
                let c_word_shuffled = self.get_shuffled_col(5).unwrap()[0..3].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 0);

                let d_word_solved = self.get_solved_diag(5).unwrap()[2..6].to_string();
                let d_word_shuffled = self.get_shuffled_diag(5).unwrap()[2..6].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 3);

                r_yellow || c_yellow || d_yellow
            }
            Cell { row: 1, col: 1 } => {
                let c_word_solved = self.get_solved_col(1).unwrap()[1..3].to_string();
                let c_word_shuffled = self.get_shuffled_col(1).unwrap()[1..3].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 0);

                let d_word_solved = self.get_solved_diag(2).unwrap();
                let d_word_shuffled = self.get_shuffled_diag(2).unwrap();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 1);

                c_yellow || d_yellow
            }
            Cell { row: 1, col: 4 } => {
                let r_word_solved = self.get_solved_row(1).unwrap()[4..6].to_string();
                let r_word_shuffled = self.get_shuffled_row(1).unwrap()[4..6].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 0);

                let c_word_solved = self.get_solved_col(4).unwrap()[0..2].to_string();
                let c_word_shuffled = self.get_shuffled_col(4).unwrap()[0..2].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 1);

                let d_word_solved = self.get_solved_diag(5).unwrap()[2..6].to_string();
                let d_word_shuffled = self.get_shuffled_diag(5).unwrap()[2..6].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 2);

                r_yellow || c_yellow || d_yellow
            }
            Cell { row: 1, col: 5 } => {
                let r_word_solved = self.get_solved_row(1).unwrap()[4..6].to_string();
                let r_word_shuffled = self.get_shuffled_row(1).unwrap()[4..6].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 1);

                let c_word_solved = self.get_solved_col(5).unwrap()[0..3].to_string();
                let c_word_shuffled = self.get_shuffled_col(5).unwrap()[0..3].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 1);

                r_yellow || c_yellow
            }
            Cell { row: 2, col: 0 } => {
                let r_word_solved = self.get_solved_row(2).unwrap()[0..4].to_string();
                let r_word_shuffled = self.get_shuffled_row(2).unwrap()[0..4].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 0);

                let c_word_solved = self.get_solved_col(0).unwrap()[2..6].to_string();
                let c_word_shuffled = self.get_shuffled_col(0).unwrap()[2..6].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 0);

                let d_word_solved = self.get_solved_diag(2).unwrap();
                let d_word_shuffled = self.get_shuffled_diag(2).unwrap();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 0);

                r_yellow || c_yellow || d_yellow
            }
            Cell { row: 2, col: 1 } => {
                let r_word_solved = self.get_solved_row(2).unwrap()[0..4].to_string();
                let r_word_shuffled = self.get_shuffled_row(2).unwrap()[0..4].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 1);

                let c_word_solved = self.get_solved_col(1).unwrap()[1..3].to_string();
                let c_word_shuffled = self.get_shuffled_col(1).unwrap()[1..3].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 1);

                let d_word_solved = self.get_solved_diag(3).unwrap()[0..2].to_string();
                let d_word_shuffled = self.get_shuffled_diag(3).unwrap()[0..2].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 1);

                r_yellow || c_yellow || d_yellow
            }
            Cell { row: 3, col: 0 } => {
                let c_word_solved = self.get_solved_col(0).unwrap()[2..6].to_string();
                let c_word_shuffled = self.get_shuffled_col(0).unwrap()[2..6].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 1);

                let d_word_solved = self.get_solved_diag(3).unwrap()[0..2].to_string();
                let d_word_shuffled = self.get_shuffled_diag(3).unwrap()[0..2].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 0);

                c_yellow || d_yellow
            }
            Cell { row: 3, col: 4 } => {
                let d_word_solved = self.get_solved_diag(7).unwrap();
                let d_word_shuffled = self.get_shuffled_diag(7).unwrap();
                is_yellow(d_word_solved, d_word_shuffled, 2)
            }
            Cell { row: 4, col: 0 } => {
                let c_word_solved = self.get_solved_col(0).unwrap()[2..6].to_string();
                let c_word_shuffled = self.get_shuffled_col(0).unwrap()[2..6].to_string();
                is_yellow(c_word_solved, c_word_shuffled, 2)
            }
            Cell { row: 4, col: 2 } => {
                let r_word_solved = self.get_solved_row(4).unwrap()[2..4].to_string();
                let r_word_shuffled = self.get_shuffled_row(4).unwrap()[2..4].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 0);

                let c_word_solved = self.get_solved_col(2).unwrap()[2..6].to_string();
                let c_word_shuffled = self.get_shuffled_col(2).unwrap()[2..6].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 2);

                let d_word_solved = self.get_solved_diag(6).unwrap()[0..2].to_string();
                let d_word_shuffled = self.get_shuffled_diag(6).unwrap()[0..2].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 1);

                r_yellow || c_yellow || d_yellow
            }
            Cell { row: 4, col: 3 } => {
                let r_word_solved = self.get_solved_row(4).unwrap()[2..4].to_string();
                let r_word_shuffled = self.get_shuffled_row(4).unwrap()[2..4].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 1);

                let d_word_solved = self.get_solved_diag(7).unwrap();
                let d_word_shuffled = self.get_shuffled_diag(7).unwrap();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 1);

                r_yellow || d_yellow
            }
            Cell { row: 5, col: 1 } => {
                let r_word_solved = self.get_solved_row(5).unwrap()[0..3].to_string();
                let r_word_shuffled = self.get_shuffled_row(5).unwrap()[0..3].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 1);

                let d_word_solved = self.get_solved_diag(6).unwrap()[0..2].to_string();
                let d_word_shuffled = self.get_shuffled_diag(6).unwrap()[0..2].to_string();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 0);

                r_yellow || d_yellow
            }
            Cell { row: 5, col: 2 } => {
                let r_word_solved = self.get_solved_row(5).unwrap()[0..3].to_string();
                let r_word_shuffled = self.get_shuffled_row(5).unwrap()[0..3].to_string();
                let r_yellow = is_yellow(r_word_solved, r_word_shuffled, 2);

                let c_word_solved = self.get_solved_col(2).unwrap()[2..6].to_string();
                let c_word_shuffled = self.get_shuffled_col(2).unwrap()[2..6].to_string();
                let c_yellow = is_yellow(c_word_solved, c_word_shuffled, 3);

                let d_word_solved = self.get_solved_diag(7).unwrap();
                let d_word_shuffled = self.get_shuffled_diag(7).unwrap();
                let d_yellow = is_yellow(d_word_solved, d_word_shuffled, 0);

                r_yellow || c_yellow || d_yellow
            }
            _ => false,
        };

        if is_yellow {
            Color::Yellow
        } else {
            Color::White
        }
    }

    // pub fn get_cell_color(&self, cell: &Cell) -> Color {
    //     if &self.get_solved_cell(cell) == &self.get_shuffled_cell(cell) {
    //         return Color::Green;
    //     };

    //     let letter = *&self.get_shuffled_cell(cell).unwrap();
    //     let solved_words = self.get_solved_words(cell);

    //     // not exactly the right logic here but it'll do for now
    //     if solved_words
    //         .into_iter()
    //         .any(|w| w.contains(&letter.to_string()))
    //     {
    //         return Color::Yellow;
    //     }

    //     Color::White
    // }

    pub fn set_shuffled_cell(&mut self, cell: &Cell, val: char) {
        self.shuffled_layout.set(cell.row, cell.col, val);
    }

    pub fn swap(&mut self, cell_a: &Cell, cell_b: &Cell) {
        let char_a = *(&self.get_shuffled_cell(cell_a).unwrap().clone());
        let char_b = *(&self.get_shuffled_cell(cell_b).unwrap().clone());

        self.set_shuffled_cell(cell_a, char_b);
        self.set_shuffled_cell(cell_b, char_a);
    }

    pub fn shuffle(&mut self, swaps_cnt: usize) {
        let swappable_cells = self.get_swappable_cells();
        for _s in 0..swaps_cnt {
            let mut swap = swappable_cells.clone();
            swap.shuffle(&mut thread_rng());
            swap = swap[0..2].to_vec();

            let cell_a = &swap[0];
            let cell_b = &swap[1];

            self.swap(cell_a, cell_b);
        }
    }
}

fn is_yellow(solved: String, shuffled: String, index: usize) -> bool {
    let letter = shuffled.chars().nth(index).unwrap();
    let mut solved = solved;
    let mut shuffled = shuffled;

    for i in 0..solved.len() {
        if solved.chars().nth(i).unwrap() == shuffled.chars().nth(i).unwrap() {
            solved = solved.replacen(solved.chars().nth(i).unwrap(), "=", 1);
            shuffled = shuffled.replacen(shuffled.chars().nth(i).unwrap(), "=", 1);
        }
    }
    for i in 0..index {
        let shuffled_char = shuffled.chars().nth(i).unwrap();
        solved = solved.replace(shuffled_char, "~");
    }

    solved.contains(letter)
}

impl fmt::Display for BeehiveSwap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "beehive-swap of size {}x{}\n", self.rows(), self.cols())?;
        write!(f, "Solved layout\n")?;
        for i in 0..self.rows() {
            let offset: String = (0..i).map(|_i| ' ').collect();
            write!(f, "{}", offset)?;
            for j in 0..self.cols() {
                write!(
                    f,
                    "{} ",
                    self.get_solved_cell(&Cell { row: i, col: j })
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
                    self.get_shuffled_cell(&Cell { row: i, col: j })
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
    use web_sys::TouchEvent;
    // use stylers::style;
    use super::*;
    #[component]
    pub fn BeehiveSwapComponent(initial_beehive: BeehiveSwap) -> impl IntoView {
        let (beehive, set_beehive) = create_signal(initial_beehive);
        let (swap_candidate, set_swap_candidate) = create_signal::<Option<Cell>>(None);
        let (cnt, cnt_set) = create_signal(0);
        let (mouse_position, set_position) = create_signal((0, 0));

        let rows = beehive.with(|bh| bh.rows());
        let cols = beehive.with(|bh| bh.cols());
        view! {
            <Show when=move || { swap_candidate.get().is_some()}>
                <div
                    class="cell dragged-cell"
                    style:left=move || format!("{}px", mouse_position.get().0 + 4)
                    style:top=move || format!("{}px", mouse_position.get().1 + 4)
                >"" {
                    let cell = swap_candidate.get().unwrap();
                    let letter = beehive.get().get_shuffled_cell(&cell).unwrap().clone();

                    letter.to_uppercase().to_string()

                } ""</div>
            </Show>
            <div
                on:mouseup=move |_| set_swap_candidate.set(None)
                on:touchend=move |_| {
                    leptos::logging::log!("clear swap");
                    set_swap_candidate.set(None)
                }
                on:touchmove=move |evt: leptos::ev::TouchEvent| {
                    let first_touch = evt.touches().item(0).expect("a touchmove event without any touches");
                    set_position.set((first_touch.client_x(), first_touch.client_y()))
                }
                on:mousemove = move |evt| {
                    set_position.set((evt.client_x(), evt.client_y()))
                }
                on:touchcancel=move |_ev| {
                    leptos::logging::log!("touchcancel on container, evt {:?}", _ev);
                }
                class="beehive-container"
                class:is-dragging= move || swap_candidate.get().is_some()
                style:width = move || format!("{}em", 2* cols + rows - 1)
                style:grid-template-columns = move || format!("repeat({}, minmax(0, 1fr))", 2* cols + rows - 1)
            >
                {
                    (0..rows).map(move |r| {
                        let offsets_before = (0..r).map(|_| view! { <div class="offset" /> }).collect_view();
                        let cells = (0..cols)
                            .map(|c| {
                                let cell = Cell{row: r, col: c};
                                let letter = move || beehive.with(|bh| bh.get_shuffled_cell(&cell).unwrap().clone());
                                let color = move || beehive.with(|bh| bh.get_cell_color(&cell));
                                let start_swapping = move || {
                                    leptos::logging::log!("start_swapping, cell {:?}, candidate: {:?}", cell, swap_candidate.get());
                                    if color() == Color::Green {
                                        return;
                                    }
                                    if swap_candidate.get().is_none() {
                                        set_swap_candidate.update(|val| *val = Some(cell));
                                    }
                                };
                                let swap = move || {
                                    leptos::logging::log!("stop_swapping, cell {:?}, candidate: {:?}", cell, swap_candidate.get());

                                    if swap_candidate.get().is_some() {
                                        let cell_a = swap_candidate.get().unwrap();
                                        let cell_b = cell;

                                        if cell_a != cell_b {
                                            set_beehive.update(|bh| bh.swap(&cell_a, &cell_b));
                                            cnt_set.update(|val| * val+=1);
                                        }
                                    }
                                };
                                let swap_with = move |cell_b: &Cell| {
                                    if swap_candidate.get().is_some() {
                                        let cell_a = &swap_candidate.get().unwrap();

                                        if cell_a != cell_b {
                                            set_beehive.update(|bh| bh.swap(&cell_a, &cell_b));
                                            cnt_set.update(|val| * val+=1);
                                        }
                                    }
                                };

                                match letter() {
                                    '_' => view! { <div class="empty-cell" /> },
                                    '\0' => view! { <div class="cell" /> },
                                    _l  => view! {
                                        <div
                                            row=r
                                            col=c
                                            on:mousedown=move |_| start_swapping()
                                            on:mouseup=move |_ev| {
                                                leptos::logging::log!("mouseup, cell: {:?}, evt {:?}", cell, _ev);
                                                swap()
                                            }
                                            on:touchstart=move |_| start_swapping()
                                            on:touchend=move |_ev: leptos::ev::TouchEvent| {
                                                // touchend target is the same as the target of the touchstart
                                                // so i cant se the same logic as with mousedown/up

                                                // get the touch position
                                                let first_touch = _ev.changed_touches().item(0).expect("a touchmove event without any touches");

                                                // get the cell over which the touchend was triggered
                                                let doc: web_sys::Document = leptos::document();
                                                let opt = doc.element_from_point(first_touch.client_x() as f32, first_touch.client_y() as f32);

                                                if let Some(elt) = opt {
                                                    // if it is a cell, swappable
                                                    let class_list =  elt.class_list();
                                                    if class_list.contains("cell") && class_list.contains("is-swappable") {
                                                        // get its row-col attributes and swap with it
                                                        let row = elt.get_attribute("row").expect("a swappable cell should have a row").parse::<usize>().expect("couldnt parse row");
                                                        let col = elt.get_attribute("col").expect("a swappable cell should have a col").parse::<usize>().expect("couldnt parse col");
                                                        let target_cell = Cell {row, col};
                                                        leptos::logging::log!("swapping cell {:?} with cell {:?}", cell, &target_cell);
                                                        swap_with(&target_cell);
                                                    }
                                                }
                                            }
                                            on:touchmove=move |_ev| {
                                                leptos::logging::log!("touchmove, cell: {:?}, evt {:?}", cell, _ev);

                                            }
                                            on:touchcancel=move |_ev| {
                                                leptos::logging::log!("touchcancel, evt {:?}, cell: {:?}", _ev, cell);
                                            }
                                            class="cell"
                                            class:is-green = move || color() == Color::Green
                                            class:is-yellow = move || color() == Color::Yellow
                                            class:is-swap = move || swap_candidate.get() == Some(cell)
                                            class:is-swappable = move || color() != Color::Green
                                        >
                                            "" {move || beehive.get().get_shuffled_cell(&cell).unwrap().clone().to_uppercase().to_string()} ""
                                        </div>
                                    },
                                }
                            }).collect_view();
                        let offsets_after = (0..(rows-r -1 )).map(|_| view! { <div class="offset" /> }).collect_view();

                        vec![offsets_before, cells, offsets_after].into_iter().collect_view()
                    }).collect_view()
                }
            </div>
            <div>
                "Swap counter: " {cnt}
            </div>
        }
    }
}

// #[test]
// fn test_shuffle() {
//     let mut grid = GridBeehive::new(6, 6);
//     grid.set_row(0, "__yeah".to_string());
//     grid.set_row(1, "_h__so".to_string());
//     grid.set_row(2, "sofa_t".to_string());
//     grid.set_row(3, "t_r_i_".to_string());
//     grid.set_row(4, "a_ex__".to_string());
//     grid.set_row(5, "the___".to_string());

//     let beehive_swap = BeehiveSwap::from(grid);

//     println!("{}", beehive_swap);

//     for cell in beehive_swap.get_swappable_cells() {
//         println!(
//             "cell: {}-{} '{}', part of words {:?}",
//             &cell.row,
//             &cell.col,
//             beehive_swap.get_solved_cell(&cell).unwrap(),
//             beehive_swap.get_solved_words(&cell)
//         );
//     }
// }
