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

    pub fn init(
        &mut self,
        oop_range: &[f32],
        ip_range: &[f32],
        flop: &[u8],
        initial_pot: i32,
        initial_stack: i32,
        oop_flop_bet_sizes: &[f32],
        oop_flop_raise_sizes: &[f32],
        oop_turn_bet_sizes: &[f32],
        oop_turn_raise_sizes: &[f32],
        oop_river_bet_sizes: &[f32],
        oop_river_raise_sizes: &[f32],
        ip_flop_bet_sizes: &[f32],
        ip_flop_raise_sizes: &[f32],
        ip_turn_bet_sizes: &[f32],
        ip_turn_raise_sizes: &[f32],
        ip_river_bet_sizes: &[f32],
        ip_river_raise_sizes: &[f32],
        add_all_in_threshold: f32,
        replace_all_in_threshold: f32,
        adjust_last_two_bet_sizes: bool,
    ) -> Option<String> {
        let bet_sizes = |bet: &[f32], raise: &[f32]| BetSizeCandidates {
            bet: bet
                .iter()
                .map(|&x| BetSize::PotRelative(x))
                .collect::<Vec<_>>(),
            raise: raise
                .iter()
                .map(|&x| BetSize::PotRelative(x))
                .collect::<Vec<_>>(),
        };

        let config = GameConfig {
            flop: flop.try_into().unwrap(),
            initial_pot,
            initial_stack,
            range: [
                Range::from_raw_data(oop_range),
                Range::from_raw_data(ip_range),
            ],
            flop_bet_sizes: [
                bet_sizes(oop_flop_bet_sizes, oop_flop_raise_sizes),
                bet_sizes(ip_flop_bet_sizes, ip_flop_raise_sizes),
            ],
            turn_bet_sizes: [
                bet_sizes(oop_turn_bet_sizes, oop_turn_raise_sizes),
                bet_sizes(ip_turn_bet_sizes, ip_turn_raise_sizes),
            ],
            river_bet_sizes: [
                bet_sizes(oop_river_bet_sizes, oop_river_raise_sizes),
                bet_sizes(ip_river_bet_sizes, ip_river_raise_sizes),
            ],
            add_all_in_threshold,
            replace_all_in_threshold,
            adjust_last_two_bet_sizes,
        };

        self.game.update_config(&config).err()
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
