extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "rayon"))]
struct DummyPool;
#[cfg(not(feature = "rayon"))]
static mut THREAD_POOL: Option<DummyPool> = None;
#[cfg(not(feature = "rayon"))]
impl DummyPool {
    fn install<OP: FnOnce() -> R, R: Default>(&self, _op: OP) -> R {
        R::default()
    }
}

#[cfg(feature = "rayon")]
mod rayon_adapter;
#[cfg(feature = "rayon")]
use rayon_adapter::THREAD_POOL;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
}

#[inline]
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
                BetSizeOptions::try_from((oop_flop_bet, oop_flop_raise)).unwrap(),
                BetSizeOptions::try_from((ip_flop_bet, ip_flop_raise)).unwrap(),
            ],
            turn_bet_sizes: [
                BetSizeOptions::try_from((oop_turn_bet, oop_turn_raise)).unwrap(),
                BetSizeOptions::try_from((ip_turn_bet, ip_turn_raise)).unwrap(),
            ],
            river_bet_sizes: [
                BetSizeOptions::try_from((oop_river_bet, oop_river_raise)).unwrap(),
                BetSizeOptions::try_from((ip_river_bet, ip_river_raise)).unwrap(),
            ],
            turn_donk_sizes: match donk_option {
                false => None,
                true => DonkSizeOptions::try_from(oop_turn_donk).ok(),
            },
            river_donk_sizes: match donk_option {
                false => None,
                true => DonkSizeOptions::try_from(oop_river_donk).ok(),
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
                if action_tree.add_line(&line).is_err() {
                    return Some("Failed to add line (loaded broken tree?)".to_string());
                }
            }
        }

        if !removed_lines.is_empty() {
            for removed_line in removed_lines.split(',') {
                let line = removed_line
                    .split(&['-', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                if action_tree.remove_line(&line).is_err() {
                    return Some("Failed to remove line (loaded broken tree?)".to_string());
                }
            }
        }

        self.game.update_config(card_config, action_tree).err()
    }

    pub fn private_cards(&self, player: usize) -> Box<[u16]> {
        let cards = self.game.private_cards(player);
        cards
            .iter()
            .map(|&(c1, c2)| c1 as u16 | ((c2 as u16) << 8))
            .collect()
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
        unsafe {
            if let Some(pool) = THREAD_POOL.as_ref() {
                pool.install(|| solve_step(&self.game, current_iteration));
            } else {
                solve_step(&self.game, current_iteration);
            }
        }
    }

    pub fn exploitability(&self) -> f32 {
        unsafe {
            if let Some(pool) = THREAD_POOL.as_ref() {
                pool.install(|| compute_exploitability(&self.game))
            } else {
                compute_exploitability(&self.game)
            }
        }
    }

    pub fn finalize(&mut self) {
        unsafe {
            if let Some(pool) = THREAD_POOL.as_ref() {
                pool.install(|| finalize(&mut self.game));
            } else {
                finalize(&mut self.game);
            }
        }
    }

    pub fn apply_history(&mut self, history: &[usize]) {
        self.game.apply_history(history);
    }

    pub fn total_bet_amount(&mut self, append: &[usize]) -> Box<[u32]> {
        if append.is_empty() {
            let total_bet_amount = self.game.total_bet_amount();
            return total_bet_amount.iter().map(|&x| x as u32).collect();
        }
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
                    Action::Bet(amount) => format!("Bet:{amount}"),
                    Action::Raise(amount) => format!("Raise:{amount}"),
                    Action::AllIn(amount) => format!("Allin:{amount}"),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join("/")
        }
    }

    pub fn actions_after(&mut self, append: &[usize]) -> String {
        if append.is_empty() {
            return self.actions();
        }
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

    pub fn get_results(&mut self) -> Box<[f64]> {
        let game = &mut self.game;
        let mut buf = Vec::new();

        let total_bet_amount = game.total_bet_amount();
        let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

        buf.push((pot_base + total_bet_amount[0]) as f64);
        buf.push((pot_base + total_bet_amount[1]) as f64);

        let trunc = |&w: &f32| if w < 0.0005 { 0.0 } else { w };
        let weights = [
            game.weights(0).iter().map(trunc).collect::<Vec<_>>(),
            game.weights(1).iter().map(trunc).collect::<Vec<_>>(),
        ];

        let is_empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
        let is_empty_flag = is_empty(0) as usize + 2 * is_empty(1) as usize;
        buf.push(is_empty_flag as f64);

        buf.extend(round_iter(weights[0].iter()));
        buf.extend(round_iter(weights[1].iter()));

        if is_empty_flag > 0 {
            buf.extend(round_iter(weights[0].iter()));
            buf.extend(round_iter(weights[1].iter()));
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

            for player in 0..2 {
                let pot = (pot_base + total_bet_amount[player]) as f64;
                for (&eq, &ev) in equity[player].iter().zip(ev[player].iter()) {
                    let (eq, ev) = (eq as f64, ev as f64);
                    if eq < 5e-7 {
                        buf.push(ev / 0.0);
                    } else {
                        buf.push(round(ev / (pot * eq)));
                    }
                }
            }
        }

        if !game.is_terminal_node() && !game.is_chance_node() {
            buf.extend(round_iter(game.strategy().iter()));
            if is_empty_flag == 0 {
                buf.extend(round_iter(
                    game.expected_values_detail(game.current_player()).iter(),
                ));
            }
        }

        buf.into_boxed_slice()
    }

    pub fn get_chance_reports(&mut self, append: &[usize], num_actions: usize) -> Box<[f64]> {
        let game = &mut self.game;
        let history = game.history().to_vec();

        let mut status = vec![0.0; 52]; // 0: not possible, 1: empty, 2: not empty
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

            let trunc = |&w: &f32| if w < 0.0005 { 0.0 } else { w };
            let weights = [
                game.weights(0).iter().map(trunc).collect::<Vec<_>>(),
                game.weights(1).iter().map(trunc).collect::<Vec<_>>(),
            ];

            combos[0][chance] = round(weights[0].iter().fold(0.0, |acc, &w| acc + w as f64));
            combos[1][chance] = round(weights[1].iter().fold(0.0, |acc, &w| acc + w as f64));

            let is_empty = |player: usize| weights[player].iter().all(|&w| w == 0.0);
            let is_empty_flag = [is_empty(0), is_empty(1)];

            game.cache_normalized_weights();
            let normalizer = [game.normalized_weights(0), game.normalized_weights(1)];

            if !game.is_terminal_node() {
                let current_player = game.current_player();
                if !is_empty_flag[current_player] {
                    let strategy_tmp = game.strategy();
                    let num_hands = game.private_cards(current_player).len();
                    let ws = if is_empty_flag[current_player ^ 1] {
                        &weights[current_player]
                    } else {
                        normalizer[current_player]
                    };
                    for action in 0..num_actions {
                        let slice = &strategy_tmp[action * num_hands..(action + 1) * num_hands];
                        let strategy_summary = weighted_average(slice, ws);
                        strategy[action * 52 + chance] = round(strategy_summary);
                    }
                }
            }

            if is_empty_flag[0] || is_empty_flag[1] {
                status[chance] = 1.0;
                game.apply_history(&history);
                continue;
            }

            status[chance] = 2.0;

            let total_bet_amount = game.total_bet_amount();
            let pot_base = game.tree_config().starting_pot + total_bet_amount.iter().min().unwrap();

            for player in 0..2 {
                let pot = (pot_base + total_bet_amount[player]) as f32;
                let equity_tmp = weighted_average(&game.equity(player), normalizer[player]);
                let ev_tmp = weighted_average(&game.expected_values(player), normalizer[player]);
                equity[player][chance] = round(equity_tmp);
                ev[player][chance] = round(ev_tmp);
                eqr[player][chance] = round(ev_tmp / (pot as f64 * equity_tmp));
            }

            game.apply_history(&history);
        }

        let mut buf = Vec::new();

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

        buf.into_boxed_slice()
    }
}
