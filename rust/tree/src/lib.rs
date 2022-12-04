extern crate wasm_bindgen;
use postflop_solver::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TreeManager {
    tree: ActionTree,
}

fn action_to_string(action: Action) -> String {
    match action {
        Action::Fold => "Fold".to_string(),
        Action::Check => "Check".to_string(),
        Action::Call => "Call".to_string(),
        Action::Bet(amount) => format!("Bet {}", amount),
        Action::Raise(amount) => format!("Raise {}", amount),
        Action::AllIn(amount) => format!("All-in {}", amount),
        _ => unreachable!(),
    }
}

fn encode_action(action: Action) -> String {
    match action {
        Action::Fold => "F".to_string(),
        Action::Check => "X".to_string(),
        Action::Call => "C".to_string(),
        Action::Bet(amount) => format!("B{}", amount),
        Action::Raise(amount) => format!("R{}", amount),
        Action::AllIn(amount) => format!("A{}", amount),
        _ => unreachable!(),
    }
}

fn encode_line(line: &[Action]) -> String {
    let mut flag = 0;
    let mut encoded = String::new();

    if line.is_empty() {
        return "(Root)".to_string();
    }

    for &action in line {
        if !encoded.is_empty() {
            let delimiter = if flag == 2 { "|" } else { ">" };
            encoded.push_str(delimiter);
        }
        match action {
            Action::Check => flag += 1,
            Action::Call => flag = 2,
            _ => flag = 0,
        }
        encoded.push_str(&encode_action(action));
    }

    encoded
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
impl TreeManager {
    pub fn new(
        board_len: i32,
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
    ) -> Self {
        let initial_state = match board_len {
            len if len <= 3 => BoardState::Flop,
            4 => BoardState::Turn,
            5 => BoardState::River,
            _ => panic!("Invalid board length"),
        };

        let config = TreeConfig {
            initial_state,
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

        let mut tree = ActionTree::new(config).unwrap();

        if !added_lines.is_empty() {
            for line in added_lines.split(',') {
                let line = line
                    .split(&['>', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                tree.add_line(&line).unwrap();
            }
        }

        if !removed_lines.is_empty() {
            for line in removed_lines.split(',') {
                let line = line
                    .split(&['>', '|'][..])
                    .map(decode_action)
                    .collect::<Vec<_>>();
                tree.remove_line(&line).unwrap();
            }
        }

        Self { tree }
    }

    pub fn added_lines(&self) -> String {
        self.tree
            .added_lines()
            .iter()
            .map(|l| encode_line(l))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn removed_lines(&self) -> String {
        self.tree
            .removed_lines()
            .iter()
            .map(|l| encode_line(l))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn invalid_terminals(&self) -> String {
        self.tree
            .invalid_terminals()
            .iter()
            .map(|l| encode_line(l))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn actions(&self) -> String {
        self.tree
            .actions()
            .iter()
            .rev()
            .cloned()
            .map(action_to_string)
            .collect::<Vec<_>>()
            .join("/")
    }

    pub fn is_terminal_action(&self) -> u32 {
        self.tree
            .is_terminal_action()
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, &x)| acc | (x as u32) << i)
    }

    pub fn is_chance_node(&self) -> bool {
        self.tree.is_chance_node()
    }

    pub fn play(&mut self, index: usize) {
        let actions = self.tree.actions();
        let action = actions[actions.len() - index - 1];
        self.tree.play(action).unwrap();
    }

    pub fn undo(&mut self) {
        self.tree.undo().unwrap();
    }

    pub fn add_bet_action(&mut self, amount: i32, is_raise: bool) {
        let action = match is_raise {
            false => Action::Bet(amount),
            true => Action::Raise(amount),
        };
        self.tree.add_action(action).unwrap();
    }

    pub fn remove_current_node(&mut self) {
        self.tree.remove_current_node().unwrap();
    }

    pub fn delete_added_line(&mut self, line: &str) {
        let line = line
            .split(&['>', '|'][..])
            .map(decode_action)
            .collect::<Vec<_>>();
        self.tree.remove_line(&line).unwrap();
    }

    pub fn delete_removed_line(&mut self, line: &str) {
        let line = line
            .split(&['>', '|'][..])
            .map(decode_action)
            .collect::<Vec<_>>();
        self.tree.add_line(&line).unwrap();
    }
}
