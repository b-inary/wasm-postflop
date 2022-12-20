extern crate wasm_bindgen;
use postflop_solver::*;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RangeManager {
    range: Range,
}

#[wasm_bindgen]
impl RangeManager {
    pub fn new() -> Self {
        Self {
            range: Range::new(),
        }
    }

    pub fn clear(&mut self) {
        self.range.clear();
    }

    pub fn update(&mut self, row: u8, col: u8, weight: f32) {
        let rank1 = 13 - row;
        let rank2 = 13 - col;
        match row.cmp(&col) {
            Ordering::Equal => self.range.set_weight_pair(rank1, weight),
            Ordering::Less => self.range.set_weight_suited(rank1, rank2, weight),
            Ordering::Greater => self.range.set_weight_offsuit(rank1, rank2, weight),
        }
    }

    pub fn from_string(&mut self, s: &str) -> Option<String> {
        let result = Range::from_sanitized_str(s);
        if let Ok(unwrap) = result {
            self.range = unwrap;
            None
        } else {
            result.err()
        }
    }

    pub fn to_string(&self) -> String {
        self.range.to_string()
    }

    pub fn get_weights(&self) -> Box<[f32]> {
        let mut weights = vec![0.0; 13 * 13];

        for row in 0..13 {
            for col in 0..13 {
                let rank1 = 12 - row as u8;
                let rank2 = 12 - col as u8;
                weights[row * 13 + col] = match row.cmp(&col) {
                    Ordering::Equal => self.range.get_weight_pair(rank1),
                    Ordering::Less => self.range.get_weight_suited(rank1, rank2),
                    Ordering::Greater => self.range.get_weight_offsuit(rank1, rank2),
                };
            }
        }

        weights.into()
    }

    pub fn raw_data(&self) -> Box<[f32]> {
        self.range.raw_data().into()
    }
}
