extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
}

#[wasm_bindgen]
impl GameManager {
    pub fn new() -> Self {
        Self {
            game: PostFlopGame::new(),
        }
    }

    pub fn init(&mut self) -> Option<String> {
        // top-40% range
        let oop_range =
            "22+,A2s+,A8o+,K7s+,K9o+,Q8s+,Q9o+,J8s+,J9o+,T8+,97+,86+,75+,64s+,65o,54,43s";
        // top-25% range
        let ip_range = "22+,A4s+,A9o+,K9s+,KTo+,Q9s+,QTo+,J9+,T9,98s,87s,76s,65s";

        let bet_sizes = bet_sizes_from_str("50%", "50%").unwrap();
        let config = GameConfig {
            flop: flop_from_str("Td9d6h").unwrap(),
            initial_pot: 60,
            initial_stack: 770,
            range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            add_all_in_threshold: 0.0,
            replace_all_in_threshold: 0.0,
            adjust_last_two_bet_sizes: false,
        };

        let result = self.game.update_config(&config);
        result.err()
    }

    pub fn memory_usage(&self, enable_compression: bool) -> u64 {
        if !enable_compression {
            self.game.memory_usage().0
        } else {
            self.game.memory_usage().1
        }
    }

    pub fn allocate_memory(&mut self, enable_compression: bool) {
        self.game.allocate_memory(enable_compression);
    }

    pub fn solve_step(&mut self, current_iteration: i32) {
        solve_step(&mut self.game, current_iteration);
    }

    pub fn exploitability(&self) -> f32 {
        compute_exploitability(&self.game, false)
    }

    pub fn normalize(&mut self) {
        normalize_strategy(&mut self.game);
    }

    pub fn ev(&self, player: usize) -> f32 {
        compute_ev(&self.game, player)
    }
}
