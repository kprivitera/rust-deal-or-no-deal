use dialoguer::Select;
use dialoguer::Confirm;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::round::Round;
use crate::game_table::GameTable;


pub struct Game {
  all_cases: HashMap<u32, f64>,
  pub all_selected_cases: Vec<u32>,
  pub case_values: HashMap<u32, f64>,
  current_round: u32,
  initial_case_choice: u32,
  rounds: Vec<Round>,
}

const AMOUNTS: [f64; 22] = [0.5, 1.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0, 750.0, 1000.0, 2500.0, 5000.0, 7500.0, 10000.0, 20000.0, 30000.0, 40000.0, 50000.0, 75000.0, 100000.0];

impl Game {
  fn create_rounds(cases_per_round: &[u32]) -> Vec<Round> {
    cases_per_round
      .iter()
      .enumerate()
      .map(|(i, &cases)| Round::new(i as u32, cases))
      .collect()
  }

  pub fn play_game(&mut self) {
    for mut round in self.rounds.clone() {
        round.play_round(self, round.number);

        // if !Confirm::new()
        //   .with_prompt("Round complete. Are you ready for the next round?")
        //   .interact()
        //   .unwrap()
        // {
        //     println!("Okay, we'll wait for you to be ready.");
        //     break;
        // }
        // calculate bank offer
    }
  }
  

  pub fn new() -> Game {
    // let original_cases = 
    let mut rng = rand::thread_rng();
    let cases_per_round = vec![6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1];
    let rounds = Self::create_rounds(&cases_per_round);
    let mut amounts: Vec<f64> = AMOUNTS.to_vec();
    amounts.shuffle(&mut rng);

    // let mut case_values = HashMap::new();
    let mut case_values: HashMap<u32, f64> = amounts.iter().enumerate()
      .map(|(i, &amount)| ((i+1) as u32, amount))
      .collect(); // the iterator of tuples is converted into a hash map with collect

    let all_cases = case_values.clone();

    // Convert the keys of the HashMap into a Vec
    let mut case_numbers: Vec<u32> = case_values.keys().cloned().collect();
    
    // Hash maps do not guarantee order, so we sort the keys
    case_numbers.sort();

    // Create the selection prompt
    let selection = Select::new()
        .with_prompt("Choose your initial case")
        .items(&case_numbers)
        .default(0)
        .interact()
        .unwrap();

    // The selected case number is
    let selected_case = case_numbers[selection];
    case_values.remove(&selected_case);

    Game {
      case_values,
      current_round: 1,
      initial_case_choice: selected_case,
      rounds,
      all_selected_cases: vec![selected_case],
      all_cases,
    }
  }

  pub fn show_game_board(&self) {
    let mut sorted_cases: Vec<(&u32, &f64)> = self.all_cases.iter().collect();
    sorted_cases.sort_by(|&(_, &case_value1), &(_, &case_value2)| case_value1.partial_cmp(&case_value2).unwrap());
    
    // let table = Game::create_table(sorted_cases, &self.all_selected_cases);
    let game_table = GameTable::new(sorted_cases, &self.all_selected_cases);
    game_table.print();
}

  pub fn calculate_bank_offer(&self) {
    //     let total: u32 = self.remaining_cases.iter().sum();
    //     let average: f64 = total as f64 / self.remaining_cases.len() as f64;

    //     // The bank offer is typically a bit less than the average
    //     average * 0.75
    }

}
