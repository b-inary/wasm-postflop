extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen-rayon")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
    result_buffer: Vec<f64>,
    report_buffer: Vec<f64>,
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
            result_buffer: Vec::new(),
            report_buffer: Vec::new(),
        }
    }

    pub fn init(
        &mut self,
        oop_range: &[f32],
        ip_range: &[f32],
        board: &[u8],
        starting_pot: i32,
        effective_stack: i32,
        rake_rate: f64,
        rake_cap: f64,
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
        add_allin_threshold: f64,
        force_allin_threshold: f64,
        merging_threshold: f64,
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
            rake_rate,
            rake_cap,
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
                    .split(&['-', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                action_tree.add_line(&line).unwrap();
            }
        }

        if !removed_lines.is_empty() {
            for removed_line in removed_lines.split(',') {
                let line = removed_line
                    .split(&['-', '|'][..])
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

    pub fn total_bet_amount(&mut self, append: &[usize]) -> Box<[u32]> {
        let history = self.game.history().to_vec();
        for &action in append {
            self.game.play(action);
        }
        let total_bet_amount = self.game.total_bet_amount();
        let ret = total_bet_amount.iter().map(|&x| x as u32).collect();
        self.game.apply_history(&history);
        ret
    }

    pub fn current_player(&self) -> String {
        if self.game.is_terminal_node() {
            "terminal".to_string()
        } else if self.game.is_chance_node() {
            "chance".to_string()
        } else if self.game.current_player() == 0 {
            "oop".to_string()
        } else {
            "ip".to_string()
        }
    }

    pub fn num_actions(&self) -> usize {
        self.game.available_actions().len()
    }

    fn actions(&self) -> String {
        if self.game.is_terminal_node() {
            "terminal".to_string()
        } else if self.game.is_chance_node() {
            "chance".to_string()
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

    pub fn actions_after_history(&mut self, append: &[usize]) -> String {
        let history = self.game.history().to_vec();
        for &action in append {
            self.game.play(action);
        }
        let ret = self.actions();
        self.game.apply_history(&history);
        ret
    }

    pub fn possible_cards(&self) -> u64 {
        self.game.possible_cards()
    }

    pub fn get_results(&mut self) -> ReadonlyBuffer {
        let game = &mut self.game;

        let buf = &mut self.result_buffer;
        buf.clear();

        let total_bet_amount = game.total_bet_amount();
        let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

        buf.push((pot_base + total_bet_amount[0]) as f64);
        buf.push((pot_base + total_bet_amount[1]) as f64);

        let weights = [game.weights(0), game.weights(1)];
        let is_empty = |weights: &[f32]| weights.iter().all(|&w| w < 0.0005);
        let is_empty_flag = is_empty(weights[0]) as usize + 2 * is_empty(weights[1]) as usize;

        buf.push(is_empty_flag as f64);
        buf.extend(round_iter(weights[0].iter()));
        buf.extend(round_iter(weights[1].iter()));

        if is_empty_flag > 0 {
            let board = game.current_board();
            let board_mask = board.iter().fold(0u64, |mask, &c| mask | (1 << c));

            for player in 0..2 {
                let normalized_weights_iter = game.private_cards(player).iter().map(|&(c1, c2)| {
                    let hand_mask = (1 << c1) | (1 << c2);
                    (hand_mask & board_mask == 0) as usize as f64
                });
                buf.extend(normalized_weights_iter);
            }
        } else {
            game.cache_normalized_weights();

            buf.extend(round_iter(game.normalized_weights(0).iter()));
            buf.extend(round_iter(game.normalized_weights(1).iter()));

            let equity = [game.equity(0), game.equity(1)];
            let ev = [game.expected_values(0), game.expected_values(1)];

            buf.extend(round_iter(equity[0].iter()));
            buf.extend(round_iter(equity[1].iter()));
            buf.extend(round_iter(ev[0].iter()));
            buf.extend(round_iter(ev[1].iter()));

            let mut eqr = [
                Vec::with_capacity(equity[0].len()),
                Vec::with_capacity(equity[1].len()),
            ];

            for player in 0..2 {
                let pot = (pot_base + total_bet_amount[player]) as f32;
                for (&eq, &ev) in equity[player].iter().zip(ev[player].iter()) {
                    if ev.abs() < 1e-6 {
                        eqr[player].push(0.0);
                    } else if eq < 1e-6 {
                        eqr[player].push(ev / 0.0);
                    } else {
                        eqr[player].push(ev / (pot * eq));
                    }
                }
            }

            buf.extend(round_iter(eqr[0].iter()));
            buf.extend(round_iter(eqr[1].iter()));
        }

        if !game.is_terminal_node() && !game.is_chance_node() {
            buf.extend(round_iter(game.strategy().iter()));
            if is_empty_flag == 0 {
                buf.extend(round_iter(
                    game.expected_values_detail(game.current_player()).iter(),
                ));
            }
        }

        ReadonlyBuffer {
            pointer: buf.as_ptr() as *const u8,
            byte_length: buf.len() * 8,
        }
    }

    pub fn get_chance_reports(&mut self, append: &[usize], num_actions: usize) -> ReadonlyBuffer {
        let game = &mut self.game;
        let history = game.history().to_vec();

        let mut status = vec![0.0; 52]; // 0: empty range, 1: not empty range
        let mut combos = [vec![0.0; 52], vec![0.0; 52]];
        let mut equity = [vec![0.0; 52], vec![0.0; 52]];
        let mut ev = [vec![0.0; 52], vec![0.0; 52]];
        let mut eqr = [vec![0.0; 52], vec![0.0; 52]];
        let mut strategy = vec![0.0; num_actions * 52];

        let possible_cards = game.possible_cards();
        for chance in 0..52 {
            if possible_cards & (1 << chance) == 0 {
                continue;
            }

            game.play(chance);
            for &action in &append[1..] {
                game.play(action);
            }

            game.cache_normalized_weights();

            let weights = [game.weights(0), game.weights(1)];
            let normalizer = [game.normalized_weights(0), game.normalized_weights(1)];

            if !game.is_terminal_node() {
                let strategy_tmp = game.strategy();
                let current_player = game.current_player();
                let num_hands = game.private_cards(current_player).len();
                for action in 0..num_actions {
                    let slice = &strategy_tmp[action * num_hands..(action + 1) * num_hands];
                    let strategy_summary = weighted_average(slice, &normalizer[current_player]);
                    strategy[chance * num_actions + action] = round(strategy_summary);
                }
            }

            let is_oop_empty = weights[0].iter().all(|&w| w < 0.0005);
            let is_ip_empty = weights[1].iter().all(|&w| w < 0.0005);

            if is_oop_empty || is_ip_empty {
                game.apply_history(&history);
                continue;
            }

            status[chance] = 1.0;

            let total_bet_amount = game.total_bet_amount();
            let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

            for player in 0..2 {
                let pot = (pot_base + total_bet_amount[player]) as f32;
                let equity_tmp = weighted_average(&game.equity(player), &normalizer[player]);
                let ev_tmp = weighted_average(&game.expected_values(player), &normalizer[player]);
                combos[player][chance] = weights[player].iter().fold(0.0, |acc, &w| acc + w as f64);
                equity[player][chance] = round(equity_tmp);
                ev[player][chance] = round(ev_tmp);
                eqr[player][chance] = round(ev_tmp / (pot as f64 * equity_tmp));
            }

            game.apply_history(&history);
        }

        let buf = &mut self.report_buffer;
        buf.clear();

        buf.extend_from_slice(&status);
        buf.extend_from_slice(&combos[0]);
        buf.extend_from_slice(&combos[1]);
        buf.extend_from_slice(&equity[0]);
        buf.extend_from_slice(&equity[1]);
        buf.extend_from_slice(&ev[0]);
        buf.extend_from_slice(&ev[1]);
        buf.extend_from_slice(&eqr[0]);
        buf.extend_from_slice(&eqr[1]);
        buf.extend_from_slice(&strategy);

        ReadonlyBuffer {
            pointer: buf.as_ptr() as *const u8,
            byte_length: buf.len() * 8,
        }
    }
}

#[inline]
fn round(value: f64) -> f64 {
    if value < 1.0 {
        (value * 1000000.0).round() / 1000000.0
    } else if value < 10.0 {
        (value * 100000.0).round() / 100000.0
    } else if value < 100.0 {
        (value * 10000.0).round() / 10000.0
    } else if value < 1000.0 {
        (value * 1000.0).round() / 1000.0
    } else if value < 10000.0 {
        (value * 100.0).round() / 100.0
    } else {
        (value * 10.0).round() / 10.0
    }
}

#[inline]
fn round_iter<'a>(iter: impl Iterator<Item = &'a f32> + 'a) -> impl Iterator<Item = f64> + 'a {
    iter.map(|&x| round(x as f64))
}

#[inline]
pub fn weighted_average(slice: &[f32], weights: &[f32]) -> f64 {
    let mut sum = 0.0;
    let mut weight_sum = 0.0;
    for (&value, &weight) in slice.iter().zip(weights.iter()) {
        sum += value as f64 * weight as f64;
        weight_sum += weight as f64;
    }
    sum / weight_sum
}
