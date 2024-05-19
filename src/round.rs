use dialoguer::Select;
use dialoguer::Confirm;
use std::collections::HashMap;
use crate::game::Game;


#[derive(Clone)]
pub struct Round {
  pub number: u32,
  cases_to_choose: u32,
  chosen_cases: Vec<u32>,
}

impl Round {
  pub fn new(number: u32, cases_to_choose: u32) -> Round {
    Round {
      number,
      cases_to_choose,
      chosen_cases: Vec::new(),
    }
  }

  pub fn play_round(&mut self, game: &mut Game, round_number: u32) {
    println!("----- Round {} -----", round_number + 1); 
    for choice_no in 0..self.cases_to_choose {
      println!("----- Choice {} -----", choice_no + 1); 
      self.choose_case(&mut game.case_values, &mut game.all_selected_cases);
      game.show_game_board();

      if !Confirm::new()
        .with_prompt("Case chosen. Choose next?")
        .interact()
        .unwrap()
      {
          println!("Okay, we'll wait for you to be ready.");
          break;
      }
    }
  }

  pub fn choose_case(&mut self, case_values: &mut HashMap<u32, f64>, all_selected_cases: &mut Vec<u32>) {
    let mut case_numbers: Vec<u32> = case_values.keys().cloned().collect();
    case_numbers.sort();

    let selection = Select::new()
      .with_prompt("Choose a case")
      .items(&case_numbers)
      .default(0)
      .interact()
      .unwrap();

    let selected_case = case_numbers[selection];
    let selected_value = case_values.get(&selected_case).unwrap();

    println!("You selected: {}", selected_case);
    println!("The amount is: {}", selected_value);
    println!("====================");

    self.chosen_cases.push(selection as u32 + 1);
    all_selected_cases.push(selected_case);
    case_values.remove(&selected_case);
  }
}