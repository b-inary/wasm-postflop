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
    }

    pub fn apply_history(&mut self, history: &[usize]) {
        self.game.apply_history(history);
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

    pub fn is_terminal_action(&self) -> u32 {
        self.game
            .is_terminal_action()
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &x)| acc | (x as u32) << i)
    }

    pub fn possible_cards(&self) -> u64 {
        self.game.possible_cards()
    }

    pub fn current_player(&self) -> usize {
        self.game.current_player()
    }

    pub fn get_results(&mut self) -> Box<[f32]> {
        let player = self.current_player();
        self.game.cache_normalized_weights();
        self.game
            .weights(player)
            .iter()
            .chain(self.game.normalized_weights(player).iter())
            .cloned()
            .chain(self.game.equity(player).into_iter())
            .chain(self.game.expected_values_detail().into_iter())
            .chain(self.game.strategy().into_iter())
            .collect()
    }

    pub fn available_actions_after_chance(&mut self) -> String {
        let history = self.game.history().to_vec();
        let possible_cards = self.possible_cards();
        let first_action = possible_cards.trailing_zeros() as usize;

        self.game.play(first_action);
        let ret = self.available_actions();
        self.game.apply_history(&history);

        ret
    }

    pub fn chance_report(&mut self) -> Box<[f32]> {
        let history = self.game.history().to_vec();
        let num_actions = self.available_actions_after_chance().split('/').count();
        let num_private_hands = self.game.num_private_hands(0);

        let possible_cards = self.possible_cards();
        let mut result = vec![0.0; 52 * (num_actions + 2)];

        for i in 0..52 {
            if possible_cards & (1 << i) != 0 {
                self.game.play(i);
                self.game.cache_normalized_weights();
                let weights = self.game.normalized_weights(0);
                let node_strategy = self.game.strategy();
                result[i] = compute_average(&self.game.equity(0), weights);
                result[i + 52] = compute_average(&self.game.expected_values(), weights);
                for j in 0..num_actions {
                    let start = j * num_private_hands;
                    let end = (j + 1) * num_private_hands;
                    result[i + (j + 2) * 52] = compute_average(&node_strategy[start..end], weights);
                }
                self.game.apply_history(&history);
            }
        }

        result.into_boxed_slice()
    }
}
