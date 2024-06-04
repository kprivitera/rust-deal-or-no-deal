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
    for (i, mut round) in self.rounds.clone().into_iter().enumerate() {
      round.play_round(self, round.number);
  
      // If it's the last round
      if i == self.rounds.len() - 1 {
          println!("This is the last round! Lets open your initial case.");
          self.all_selected_cases.push(self.initial_case_choice);
          self.show_game_board();
      }  
    }
  }

  fn create_case_values() -> HashMap<u32, f64> {
    let mut rng = rand::thread_rng();
    let mut amounts: Vec<f64> = AMOUNTS.to_vec();
    amounts.shuffle(&mut rng);

    let case_values: HashMap<u32, f64> = amounts.iter().enumerate()
      .map(|(i, &amount)| ((i+1) as u32, amount))
      .collect(); // the iterator of tuples is converted into a hash map with collect

    case_values
  }

  fn choose_initial_case(case_numbers: &Vec<u32>) -> Option<u32> {
    let selection = match Select::new()
      .with_prompt("Choose your initial case")
      .items(&case_numbers)
      .default(0)
      .interact() {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to make a selection");
            return None;
        }
    };
    Some(case_numbers[selection])
  }
  

  pub fn new() -> Game {
    let cases_per_round = vec![6, 5, 3, 2, 2, 1, 1];
    let rounds = Self::create_rounds(&cases_per_round);
    let mut case_values = Self::create_case_values();
    let all_cases = case_values.clone();

    // Convert the keys of the HashMap into a Vec
    let mut case_numbers: Vec<u32> = case_values.keys().cloned().collect();
    
    // Hash maps do not guarantee order, so we sort the keys
    case_numbers.sort();

    // Create the selection prompt
    let selected_case = Self::choose_initial_case(&case_numbers)
      .expect("Failed to select an initial case");

    case_values.remove(&selected_case);

    Game {
        case_values,
        current_round: 1,
        initial_case_choice: selected_case,
        rounds,
        all_selected_cases: Vec::new(),
        all_cases,
    }
  }

  pub fn show_game_board(&self) {
    let mut sorted_cases: Vec<(&u32, &f64)> = self.all_cases.iter().collect();
    sorted_cases.sort_by(|&(_, &case_value1), &(_, &case_value2)| case_value1.partial_cmp(&case_value2).unwrap());
    
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
