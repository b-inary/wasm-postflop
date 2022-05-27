extern crate wasm_bindgen;
use postflop_solver::*;
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
        if row == col {
            self.range.set_weight_pair(rank1, weight).unwrap();
        } else if row < col {
            self.range.set_weight_suited(rank1, rank2, weight).unwrap();
        } else {
            self.range.set_weight_offsuit(rank1, rank2, weight).unwrap();
        }
    }

    pub fn from_string(&mut self, s: &str) -> Option<String> {
        let result = Range::from_sanitized_ranges(s);
        if result.is_ok() {
            self.range = result.unwrap();
            return None;
        } else {
            return result.err();
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
                if row == col {
                    weights[row * 13 + col] = self.range.get_weight_pair(rank1);
                } else if row < col {
                    weights[row * 13 + col] = self.range.get_weight_suited(rank1, rank2);
                } else {
                    weights[row * 13 + col] = self.range.get_weight_offsuit(rank1, rank2);
                }
            }
        }

        weights.into()
    }

    pub fn raw_data(&self) -> Box<[f32]> {
        self.range.raw_data().into()
    }
}
