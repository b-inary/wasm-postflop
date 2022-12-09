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

fn decode_action(action: &str) -> Action {
    match action {
        "F" => Action::Fold,
        "X" => Action::Check,
        "C" => Action::Call,
        _ => {
            let mut chars = action.chars();
            let first_char = chars.next().unwrap();
            let amount = chars.as_str().parse().unwrap();
            match first_char {
                'B' => Action::Bet(amount),
                'R' => Action::Raise(amount),
                'A' => Action::AllIn(amount),
                _ => unreachable!(),
            }
        }
    }
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
        donk_option: bool,
        oop_flop_bet: &str,
        oop_flop_raise: &str,
        oop_turn_bet: &str,
        oop_turn_raise: &str,
        oop_turn_donk: &str,
        oop_river_bet: &str,
        oop_river_raise: &str,
        oop_river_donk: &str,
        ip_flop_bet: &str,
        ip_flop_raise: &str,
        ip_turn_bet: &str,
        ip_turn_raise: &str,
        ip_river_bet: &str,
        ip_river_raise: &str,
        add_allin_threshold: f32,
        force_allin_threshold: f32,
        merging_threshold: f32,
        added_lines: &str,
        removed_lines: &str,
    ) -> Option<String> {
        let (turn, river, state) = match board.len() {
            3 => (NOT_DEALT, NOT_DEALT, BoardState::Flop),
            4 => (board[3], NOT_DEALT, BoardState::Turn),
            5 => (board[3], board[4], BoardState::River),
            _ => return Some("Invalid board length".to_string()),
        };

        let card_config = CardConfig {
            range: [
                Range::from_raw_data(oop_range).unwrap(),
                Range::from_raw_data(ip_range).unwrap(),
            ],
            flop: board[..3].try_into().unwrap(),
            turn,
            river,
        };

        let tree_config = TreeConfig {
            initial_state: state,
            starting_pot,
            effective_stack,
            flop_bet_sizes: [
                BetSizeCandidates::try_from((oop_flop_bet, oop_flop_raise)).unwrap(),
                BetSizeCandidates::try_from((ip_flop_bet, ip_flop_raise)).unwrap(),
            ],
            turn_bet_sizes: [
                BetSizeCandidates::try_from((oop_turn_bet, oop_turn_raise)).unwrap(),
                BetSizeCandidates::try_from((ip_turn_bet, ip_turn_raise)).unwrap(),
            ],
            river_bet_sizes: [
                BetSizeCandidates::try_from((oop_river_bet, oop_river_raise)).unwrap(),
                BetSizeCandidates::try_from((ip_river_bet, ip_river_raise)).unwrap(),
            ],
            turn_donk_sizes: match donk_option {
                false => None,
                true => DonkSizeCandidates::try_from(oop_turn_donk).ok(),
            },
            river_donk_sizes: match donk_option {
                false => None,
                true => DonkSizeCandidates::try_from(oop_river_donk).ok(),
            },
            add_allin_threshold,
            force_allin_threshold,
            merging_threshold,
        };

        let mut action_tree = ActionTree::new(tree_config).unwrap();

        if !added_lines.is_empty() {
            for added_line in added_lines.split(',') {
                let line = added_line
                    .split(&['>', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                action_tree.add_line(&line).unwrap();
            }
        }

        if !removed_lines.is_empty() {
            for removed_line in removed_lines.split(',') {
                let line = removed_line
                    .split(&['>', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                action_tree.remove_line(&line).unwrap();
            }
        }

        self.game.update_config(card_config, action_tree).err()
    }

    pub fn private_cards(&self, player: usize) -> ReadonlyBuffer {
        let cards = self.game.private_cards(player);
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

    pub fn actions(&self) -> String {
        if self.game.is_terminal_node() {
            "Terminal".to_string()
        } else if self.game.is_chance_node() {
            "Chance".to_string()
        } else {
            self.game
                .available_actions()
                .iter()
                .map(|&x| match x {
                    Action::Fold => "Fold:0".to_string(),
                    Action::Check => "Check:0".to_string(),
                    Action::Call => "Call:0".to_string(),
                    Action::Bet(size) => format!("Bet:{}", size),
                    Action::Raise(size) => format!("Raise:{}", size),
                    Action::AllIn(size) => format!("Allin:{}", size),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join("/")
        }
    }

    pub fn actions_after_chance(&mut self) -> String {
        let history = self.game.history().to_vec();
        self.game.play(usize::MAX);
        let ret = self.actions();
        self.game.apply_history(&history);
        ret
    }

    pub fn possible_cards(&self) -> u64 {
        self.game.possible_cards()
    }

    pub fn equity(&mut self, player: usize) -> f32 {
        self.game.cache_normalized_weights();
        let weights = self.game.normalized_weights(player);
        let equity = self.game.equity(player);
        compute_average(&equity, weights)
    }
}
