extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen-rayon")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
}

#[wasm_bindgen]
pub struct ReadonlyBuffer {
    pub pointer: *const u8,
    pub byte_length: usize,
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
        board: &[u8],
        starting_pot: i32,
        effective_stack: i32,
        oop_flop_bet: &[f32],
        oop_flop_raise: &[f32],
        oop_turn_bet: &[f32],
        oop_turn_raise: &[f32],
        oop_river_bet: &[f32],
        oop_river_raise: &[f32],
        ip_flop_bet: &[f32],
        ip_flop_raise: &[f32],
        ip_turn_bet: &[f32],
        ip_turn_raise: &[f32],
        ip_river_bet: &[f32],
        ip_river_raise: &[f32],
        add_all_in_threshold: f32,
        force_all_in_threshold: f32,
        adjust_last_two_bet_sizes: bool,
    ) -> Option<String> {
        let turn = if board.len() >= 4 {
            board[3]
        } else {
            NOT_DEALT
        };

        let river = if board.len() == 5 {
            board[4]
        } else {
            NOT_DEALT
        };

        let convert_bet_sizes = |sizes: &[f32]| {
            sizes
                .iter()
                .map(|&x| {
                    if x >= 0.0 {
                        BetSize::PotRelative(x)
                    } else {
                        BetSize::LastBetRelative(-x)
                    }
                })
                .collect()
        };

        let bet_sizes = |bet: &[f32], raise: &[f32]| BetSizeCandidates {
            bet: convert_bet_sizes(bet),
            raise: convert_bet_sizes(raise),
        };

        let config = GameConfig {
            flop: board[..3].try_into().unwrap(),
            turn,
            river,
            starting_pot,
            effective_stack,
            range: [
                Range::from_raw_data(oop_range),
                Range::from_raw_data(ip_range),
            ],
            flop_bet_sizes: [
                bet_sizes(oop_flop_bet, oop_flop_raise),
                bet_sizes(ip_flop_bet, ip_flop_raise),
            ],
            turn_bet_sizes: [
                bet_sizes(oop_turn_bet, oop_turn_raise),
                bet_sizes(ip_turn_bet, ip_turn_raise),
            ],
            river_bet_sizes: [
                bet_sizes(oop_river_bet, oop_river_raise),
                bet_sizes(ip_river_bet, ip_river_raise),
            ],
            add_all_in_threshold,
            force_all_in_threshold,
            adjust_last_two_bet_sizes,
        };

        self.game.update_config(&config).err()
    }

    pub fn private_hand_cards(&self, player: usize) -> ReadonlyBuffer {
        let cards = self.game.private_hand_cards(player);
        ReadonlyBuffer {
            pointer: cards.as_ptr() as *const u8,
            byte_length: cards.len() * 2,
        }
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

    pub fn solve_step(&self, current_iteration: u32) {
        solve_step(&self.game, current_iteration);
    }

    pub fn exploitability(&self) -> f32 {
        compute_exploitability(&self.game)
    }

    pub fn finalize(&mut self) {
        finalize(&mut self.game);
        self.game.cache_normalized_weights();
    }

    pub fn apply_history(&mut self, history: &[u32]) {
        self.game.back_to_root();
        for &action in history {
            self.game.play(action as usize);
        }
        self.game.cache_normalized_weights();
    }

    pub fn available_actions(&self) -> String {
        if self.game.is_chance_node() {
            "Chance".to_string()
        } else {
            self.game
                .available_actions()
                .iter()
                .map(|&x| match x {
                    Action::Fold => "Fold".to_string(),
                    Action::Check => "Check".to_string(),
                    Action::Call => "Call".to_string(),
                    Action::Bet(size) => format!("Bet {}", size),
                    Action::Raise(size) => format!("Raise {}", size),
                    Action::AllIn(size) => format!("All-in {}", size),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join("/")
        }
    }

    pub fn is_terminal_action(&self) -> Box<[u8]> {
        self.game
            .is_terminal_action()
            .iter()
            .map(|&x| x as u8)
            .collect()
    }

    pub fn possible_cards(&self) -> u64 {
        self.game.possible_cards()
    }

    pub fn current_player(&self) -> usize {
        self.game.current_player()
    }

    pub fn get_results(&self) -> Box<[f32]> {
        let player = self.current_player();
        self.game
            .weights(player)
            .iter()
            .cloned()
            .chain(
                self.game
                    .normalized_weights(player)
                    .iter()
                    .map(|&x| x as f32),
            )
            .chain(self.game.expected_values().into_iter())
            .chain(self.game.equity().into_iter())
            .chain(self.game.strategy().into_iter())
            .collect()
    }
}
