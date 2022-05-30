extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen-rayon")]
pub use wasm_bindgen_rayon::init_thread_pool;

struct HistoryNode {
    node: *const PostFlopNode,
    board: Vec<u8>,
    weights: [Vec<f64>; 2],
    weights_normalized: [Vec<f64>; 2],
    // chance_factor: f64,
}

impl HistoryNode {
    fn compute_normalized_weight(&mut self, private_hand_cards: &[&[(u8, u8)]; 2]) {
        self.weights_normalized[0].fill(0.0);
        self.weights_normalized[1].fill(0.0);

        let mut weight_sum = 0.0;
        let board_mask: u64 = self.board.iter().fold(0, |acc, &card| acc | 1 << card);

        for (i, &(c1, c2)) in private_hand_cards[0].iter().enumerate() {
            let oop_mask: u64 = (1 << c1) | (1 << c2);
            let oop_weight = self.weights[0][i];
            if board_mask & oop_mask == 0 && oop_weight > 0.0 {
                for (j, &(c3, c4)) in private_hand_cards[1].iter().enumerate() {
                    let ip_mask: u64 = (1 << c3) | (1 << c4);
                    let ip_weight = self.weights[1][j];
                    if (board_mask | oop_mask) & ip_mask == 0 && ip_weight > 0.0 {
                        let weight = oop_weight as f64 * ip_weight as f64;
                        self.weights_normalized[0][i] += weight;
                        self.weights_normalized[1][j] += weight;
                        weight_sum += weight;
                    }
                }
            }
        }

        for player in 0..2 {
            self.weights_normalized[player].iter_mut().for_each(|w| {
                *w /= weight_sum;
            });
        }
    }
}

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
    history: Vec<HistoryNode>,
}

#[wasm_bindgen]
impl GameManager {
    pub fn new() -> Self {
        Self {
            game: PostFlopGame::new(),
            history: vec![],
        }
    }

    pub fn init(
        &mut self,
        oop_range: &[f32],
        ip_range: &[f32],
        flop: &[u8],
        starting_pot: i32,
        effective_stack: i32,
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
            starting_pot,
            effective_stack,
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

    pub fn private_hand_cards(&self, player: usize) -> Box<[u8]> {
        self.game
            .private_hand_cards(player)
            .iter()
            .flat_map(|&(c1, c2)| vec![c1, c2])
            .collect::<Vec<_>>()
            .into_boxed_slice()
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

    pub fn solve_step(&self, current_iteration: i32) {
        solve_step(&self.game, current_iteration);
    }

    pub fn exploitability(&self) -> f32 {
        compute_exploitability(&self.game, false)
    }

    pub fn finalize(&mut self) {
        normalize_strategy(&self.game);
        compute_ev(&self.game);

        let mut history_node = HistoryNode {
            node: &*self.game.root(),
            board: self.game.config().flop.to_vec(),
            weights: [
                self.game
                    .initial_reach(0)
                    .iter()
                    .map(|&x| x as f64)
                    .collect::<Vec<_>>(),
                self.game
                    .initial_reach(1)
                    .iter()
                    .map(|&x| x as f64)
                    .collect::<Vec<_>>(),
            ],
            weights_normalized: [
                vec![0.0; self.game.num_private_hands(0)],
                vec![0.0; self.game.num_private_hands(1)],
            ],
            // chance_factor: 1.0,
        };

        history_node.compute_normalized_weight(&[
            self.game.private_hand_cards(0),
            self.game.private_hand_cards(1),
        ]);

        self.history.push(history_node);
    }

    fn current_node(&self) -> &PostFlopNode {
        unsafe { &*self.history.last().unwrap().node }
    }

    pub fn current_player(&self) -> usize {
        self.current_node().player()
    }

    pub fn get_actions(&self) -> String {
        self.current_node()
            .get_actions()
            .iter()
            .map(|&x| match x {
                Action::None => unreachable!(),
                Action::Fold => "Fold".to_string(),
                Action::Check => "Check".to_string(),
                Action::Call => "Call".to_string(),
                Action::Bet(size) => format!("Bet {}", size),
                Action::Raise(size) => format!("Raise {}", size),
                Action::AllIn(size) => format!("All-in {}", size),
                Action::Chance(card) => format!("{}", card),
            })
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn is_terminal(&self) -> Box<[i32]> {
        let node = self.current_node();
        node.actions()
            .map(|x| node.play(x).is_terminal() as i32)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get_weights(&self) -> Box<[f64]> {
        let player = self.current_node().player();
        self.history.last().unwrap().weights[player]
            .clone()
            .into_boxed_slice()
    }

    pub fn get_normalized_weights(&self) -> Box<[f64]> {
        let player = self.current_node().player();
        self.history.last().unwrap().weights_normalized[player]
            .clone()
            .into_boxed_slice()
    }

    pub fn get_expected_values(&self) -> Box<[f32]> {
        let node = self.current_node();
        if !self.game.is_compression_enabled() {
            node.cum_regret()
                .iter()
                .take(self.game.num_private_hands(node.player()))
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice()
        } else {
            let decoder = node.cum_regret_scale() / i16::MAX as f32;
            node.cum_regret_compressed()
                .iter()
                .take(self.game.num_private_hands(node.player()))
                .map(|&x| x as f32 * decoder)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        }
    }

    pub fn get_strategy(&self) -> Box<[f32]> {
        let node = self.current_node();
        if !self.game.is_compression_enabled() {
            node.strategy().to_vec().into_boxed_slice()
        } else {
            let decoder = node.strategy_scale() / u16::MAX as f32;
            node.strategy_compressed()
                .iter()
                .map(|&x| x as f32 * decoder)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        }
    }
}
