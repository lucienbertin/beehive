use serde::{Deserialize, Serialize};
use rand::{seq::SliceRandom, thread_rng};

use crate::grid_beehive::GridBeehive;
use crate::beehive_swap::BeehiveSwap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeehiveSerde {
    layout: Vec<String>,
}

impl From<GridBeehive> for BeehiveSerde {
    fn from(value: GridBeehive) -> Self {
        let mut layout = vec![];
        let rows = value.rows();
        for r in 0..rows {
            layout.push(value.get_row(r).unwrap());
        }

        Self {
            layout
        }
    }
}

impl Into<GridBeehive> for BeehiveSerde {
    fn into(self) -> GridBeehive {
        let rows = self.layout.len();
        let cols = match rows {
            0 => 0,
            _ => self.layout.get(0).unwrap().len(),
        };

        let mut beehive = GridBeehive::new(rows, cols);
        for row in 0..rows {
            beehive.set_row(row, self.layout.get(row).unwrap().clone());
        };

        beehive
    }
}

impl Into<BeehiveSwap> for BeehiveSerde {
    fn into(self) -> BeehiveSwap {
        let grid: GridBeehive = self.into();

        BeehiveSwap::from(grid)
    }
}

pub fn append_file(beehive: GridBeehive) -> std::result::Result<(), ()> {
    let bh = BeehiveSerde::from(beehive);
    // let json = serde_json::to_string(bh);
    let path = "./assets/beehives.json";
    let f = std::fs::File::open(path).map_err(|_| ())?;
    let reader = std::io::BufReader::new(f);
    let mut beehives: Vec<BeehiveSerde> = serde_json::from_reader(reader).map_err(|_| ())?;

    beehives.push(bh);

    let serialized = serde_json::to_string(&beehives).map_err(|_| ())?;

    use std::io::Write;
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(path).map_err(|_| ())?;
    file.write(serialized.as_bytes()).map_err(|_| ())?;

    Ok(())
}

pub async fn fetch_beehive(_c: ()) -> Result<BeehiveSerde, ()> {
    // make the request
    let mut beehives: Vec<BeehiveSerde> = reqwasm::http::Request::get("/assets/beehives.json")
        .send()
        .await
        .map_err(|e| ())?
        // convert it to JSON
        .json()
        .await
        .map_err(|e| ())?;

    beehives.shuffle(&mut thread_rng());

    match beehives.get(0) {
        Some(bh) => Ok(bh.clone()),
        None => Err(())
    }
}

#[test]
fn test_append() -> std::result::Result<(), ()> {
    let mut grid = GridBeehive::new(6, 6);
    grid.set_row(0, "__yeah".to_string());
    grid.set_row(1, "_h__so".to_string());
    grid.set_row(2, "sofa_t".to_string());
    grid.set_row(3, "t_r_i_".to_string());
    grid.set_row(4, "a_ex__".to_string());
    grid.set_row(5, "the___".to_string());

    append_file(grid)
}