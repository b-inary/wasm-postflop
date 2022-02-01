extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
    err_string: Option<String>,
}

#[wasm_bindgen]
impl GameManager {
    pub fn new() -> Self {
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
            max_num_bet: 5,
        };

        let game = PostFlopGame::new(&config, None);

        if game.is_ok() {
            Self {
                game: game.unwrap(),
                err_string: None,
            }
        } else {
            Self {
                game: Default::default(),
                err_string: game.err(),
            }
        }
    }

    pub fn get_err_string(&self) -> Option<String> {
        self.err_string.clone()
    }

    pub fn solve_step(&self, current_iteration: i32) {
        solve_step(&self.game, current_iteration);
    }

    pub fn exploitability(&self) -> f32 {
        compute_exploitability(&self.game, false)
    }

    pub fn normalize(&self) {
        normalize_strategy(&self.game);
    }

    pub fn ev(&self, player: usize) -> f32 {
        compute_ev(&self.game, player)
    }
}
