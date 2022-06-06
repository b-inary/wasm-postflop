extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen-rayon")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub struct GameManager {
    game: PostFlopGame,
    node: *const PostFlopNode,
    board: Vec<u8>,
    turn_swapped_suit: Option<(u8, u8)>,
    turn_swap: *const [Vec<(usize, usize)>; 2],
    river_swap: *const [Vec<(usize, usize)>; 2],
    weights: [Vec<f32>; 2],
    weights_normalized: [Vec<f32>; 2],
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
            node: std::ptr::null(),
            board: Vec::new(),
            turn_swapped_suit: None,
            turn_swap: std::ptr::null(),
            river_swap: std::ptr::null(),
            weights: Default::default(),
            weights_normalized: Default::default(),
        }
    }

    pub fn init(
        &mut self,
        oop_range: &[f32],
        ip_range: &[f32],
        flop: &[u8],
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
        replace_all_in_threshold: f32,
        adjust_last_two_bet_sizes: bool,
    ) -> Option<String> {
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
            flop: flop.try_into().unwrap(),
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
            replace_all_in_threshold,
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
        compute_exploitability(&self.game, false)
    }

    pub fn finalize(&mut self) -> f64 {
        normalize_strategy(&self.game);
        compute_ev_and_equity(&self.game);

        self.node = &*self.game.root();
        self.board = self.game.config().flop.to_vec();
        self.turn_swapped_suit = None;
        self.turn_swap = std::ptr::null();
        self.river_swap = std::ptr::null();
        self.weights = [
            self.game.initial_weight(0).to_vec(),
            self.game.initial_weight(1).to_vec(),
        ];
        self.compute_normalized_weight();
        self.weights_normalized[0]
            .iter()
            .fold(0.0, |acc, &x| acc + x as f64)
    }

    pub fn apply_history(&mut self, history: &[u32]) {
        self.node = &*self.game.root();
        self.board = self.game.config().flop.to_vec();
        self.turn_swapped_suit = None;
        self.turn_swap = std::ptr::null();
        self.river_swap = std::ptr::null();
        self.weights = [
            vec![1.0; self.game.num_private_hands(0)],
            vec![1.0; self.game.num_private_hands(1)],
        ];
        self.apply_history_recursive(history);
        mul_slice(&mut self.weights[0], &self.game.initial_weight(0));
        mul_slice(&mut self.weights[1], &self.game.initial_weight(1));
        self.compute_normalized_weight();
    }

    fn apply_history_recursive(&mut self, history: &[u32]) {
        if history.is_empty() {
            return;
        }

        let node = unsafe { &*self.node };
        let action = history[0] as usize;

        if node.is_chance() {
            let is_turn = self.board.len() == 3;

            let card = if let Some((suit1, suit2)) = self.turn_swapped_suit {
                if action as u8 & 3 == suit1 {
                    action as u8 - suit1 + suit2
                } else if action as u8 & 3 == suit2 {
                    action as u8 + suit1 - suit2
                } else {
                    action as u8
                }
            } else {
                action as u8
            };

            let mut index = usize::MAX;
            let mut isomorphic_index = usize::MAX;

            let actions = node.get_actions();
            for i in 0..actions.len() {
                if actions[i] == Action::Chance(card) {
                    index = i;
                    break;
                }
            }

            if index == usize::MAX {
                let isomorphism = self.game.isomorphic_chances(node);
                let isomorphic_card = self.game.isomorphic_card(node);
                for i in 0..isomorphism.len() {
                    if isomorphic_card[i] == card {
                        index = isomorphism[i];
                        isomorphic_index = i;
                        if self.board.len() == 3 {
                            if let Action::Chance(card2) = actions[index] {
                                self.turn_swapped_suit = Some((card & 3, card2 & 3));
                            }
                        }
                        break;
                    }
                }
            }

            self.node = &*node.play(index);
            self.board.push(action as u8);
            self.apply_history_recursive(&history[1..]);

            if isomorphic_index != usize::MAX {
                let swap_list = self.game.isomorphic_swap(node, isomorphic_index);
                for player in 0..2 {
                    for &(i, j) in &swap_list[player] {
                        self.weights[player].swap(i, j);
                    }
                }
                if is_turn {
                    self.turn_swap = swap_list;
                } else {
                    self.river_swap = swap_list;
                }
            }
        } else {
            self.node = &*node.play(action);
            self.apply_history_recursive(&history[1..]);

            if node.num_actions() > 1 {
                let player = node.player();
                if !self.game.is_compression_enabled() {
                    mul_slice(
                        &mut self.weights[player],
                        row(
                            node.strategy(),
                            action as usize,
                            self.game.num_private_hands(player),
                        ),
                    );
                } else {
                    let decoder = node.strategy_scale() / u16::MAX as f32;
                    self.weights[player]
                        .iter_mut()
                        .zip(row(
                            node.strategy_compressed(),
                            action as usize,
                            self.game.num_private_hands(player),
                        ))
                        .for_each(|(w, &s)| *w *= s as f32 * decoder);
                }
            }
        }
    }

    fn compute_normalized_weight(&mut self) {
        self.weights_normalized = [
            vec![0.0; self.game.num_private_hands(0)],
            vec![0.0; self.game.num_private_hands(1)],
        ];

        let private_hand_cards = [
            self.game.private_hand_cards(0),
            self.game.private_hand_cards(1),
        ];

        let board_mask: u64 = self.board.iter().fold(0, |acc, &card| acc | 1 << card);

        for (i, &(c1, c2)) in private_hand_cards[0].iter().enumerate() {
            let oop_mask: u64 = (1 << c1) | (1 << c2);
            let oop_weight = self.weights[0][i];
            if board_mask & oop_mask == 0 && oop_weight > 0.0 {
                for (j, &(c3, c4)) in private_hand_cards[1].iter().enumerate() {
                    let ip_mask: u64 = (1 << c3) | (1 << c4);
                    let ip_weight = self.weights[1][j];
                    if (board_mask | oop_mask) & ip_mask == 0 && ip_weight > 0.0 {
                        let weight = oop_weight * ip_weight;
                        self.weights_normalized[0][i] += weight;
                        self.weights_normalized[1][j] += weight;
                    }
                }
            }
        }
    }

    fn node(&self) -> &PostFlopNode {
        unsafe { &*self.node }
    }

    pub fn get_actions(&self) -> String {
        self.node()
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
                Action::Chance(_) => format!("Chance"),
            })
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn is_terminal_action(&self) -> Box<[u8]> {
        let node = self.node();
        node.actions()
            .map(|x| {
                let child = node.play(x);
                (child.is_terminal() || child.amount() == self.game.config().effective_stack) as u8
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn is_possible_chance(&self) -> Box<[u8]> {
        let mut mask: u64 = (1 << 52) - 1;
        let board_mask: u64 = self.board.iter().fold(0, |acc, &card| acc | 1 << card);

        let private_hand_cards = [
            self.game.private_hand_cards(0),
            self.game.private_hand_cards(1),
        ];

        for (i, &(c1, c2)) in private_hand_cards[0].iter().enumerate() {
            let oop_mask: u64 = (1 << c1) | (1 << c2);
            let oop_weight = self.weights[0][i];
            if board_mask & oop_mask == 0 && oop_weight >= 0.05 / 100.0 {
                for (j, &(c3, c4)) in private_hand_cards[1].iter().enumerate() {
                    let ip_mask: u64 = (1 << c3) | (1 << c4);
                    let ip_weight = self.weights[1][j];
                    if (board_mask | oop_mask) & ip_mask == 0 && ip_weight >= 0.05 / 100.0 {
                        mask &= board_mask | oop_mask | ip_mask;
                    }
                }
            }
        }

        (0..52)
            .map(|card| ((mask & (1 << card)) == 0) as u8)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn current_player(&self) -> usize {
        self.node().player()
    }

    pub fn get_results(&self) -> Box<[f32]> {
        let player = self.current_player();
        self.weights[player]
            .iter()
            .chain(self.weights_normalized[player].iter())
            .cloned()
            .chain(self.get_expected_values().into_iter())
            .chain(self.get_equity().into_iter())
            .chain(self.get_strategy().into_iter())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn get_expected_values(&self) -> Vec<f32> {
        let node = self.node();
        let player = node.player();
        let num_private_hands = self.game.num_private_hands(player);
        let mut ret = if !self.game.is_compression_enabled() {
            node.cum_regret()
                .iter()
                .take(num_private_hands)
                .cloned()
                .collect::<Vec<_>>()
        } else {
            let decoder = node.cum_regret_scale() / i16::MAX as f32;
            node.cum_regret_compressed()
                .iter()
                .take(num_private_hands)
                .map(|&x| x as f32 * decoder)
                .collect::<Vec<_>>()
        };
        for swap in &[self.river_swap, self.turn_swap] {
            if !swap.is_null() {
                for &(i, j) in unsafe { &(**swap)[player] } {
                    ret.swap(i, j);
                }
            }
        }
        ret
    }

    fn get_equity(&self) -> Vec<f32> {
        let node = self.node();
        let player = node.player();
        let num_actions = node.num_actions();
        let num_private_hands = self.game.num_private_hands(player);
        if !self.game.is_compression_enabled() {
            if num_actions == 1 {
                node.strategy().to_vec()
            } else {
                row(node.cum_regret(), 1, num_private_hands).to_vec()
            }
        } else {
            let decoder = node.equity_scale() / i16::MAX as f32;
            if num_actions == 1 {
                node.strategy_compressed()
                    .iter()
                    .map(|&x| x as i16 as f32 * decoder)
                    .collect()
            } else {
                row(node.cum_regret_compressed(), 1, num_private_hands)
                    .iter()
                    .map(|&x| x as f32 * decoder)
                    .collect()
            }
        }
    }

    fn get_strategy(&self) -> Vec<f32> {
        let node = self.node();
        let player = node.player();
        let num_actions = node.num_actions();
        let num_private_hands = self.game.num_private_hands(player);
        let mut ret = if num_actions == 1 {
            vec![1.0; num_private_hands]
        } else if !self.game.is_compression_enabled() {
            node.strategy().to_vec()
        } else {
            let decoder = node.strategy_scale() / u16::MAX as f32;
            node.strategy_compressed()
                .iter()
                .map(|&x| x as f32 * decoder)
                .collect::<Vec<_>>()
        };
        for swap in &[self.river_swap, self.turn_swap] {
            if !swap.is_null() {
                for &(i, j) in unsafe { &(**swap)[player] } {
                    for action in 0..num_actions {
                        ret.swap(
                            action * num_private_hands + i,
                            action * num_private_hands + j,
                        );
                    }
                }
            }
        }
        ret
    }
}
