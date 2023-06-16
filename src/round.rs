use std::collections::HashMap;
use std::sync::Arc;

use twilight_model::id::{marker::UserMarker, Id};

use crate::criteria;
use crate::words;

pub struct Round {
  starting_scores: HashMap<Id<UserMarker>, u32>,
  updated_scores: HashMap<Id<UserMarker>, u32>,
  scored_words: HashMap<Id<UserMarker>, Vec<String>>,
  use_count: HashMap<String, u32>,
  criteria: Vec<criteria::Criteron>,
  num_players: u32,
}

impl Round {
  pub fn new(initial_scores: HashMap<Id<UserMarker>, u32>) -> Self {
    let mut updated_scores: HashMap<Id<UserMarker>, u32> = HashMap::new();
    let mut scored_words: HashMap<Id<UserMarker>, Vec<String>> = HashMap::new();
    let num_players = initial_scores.len() as u32;

    for player_id in initial_scores.keys() {
      updated_scores.insert(*player_id, *initial_scores.get(player_id).unwrap());
      scored_words.insert(*player_id, vec![]);
    }

    Self {
      starting_scores: initial_scores,
      updated_scores,
      scored_words,
      use_count: HashMap::new(),
      criteria: criteria::generate_random_criteria(),
      num_players,
    }
  }

  pub fn get_criteria_string(&self) -> String {
    self.criteria.iter().map(|criteron| format!("\n- {}", criteron.to_string())).collect::<Vec<_>>().join("")
  }

  pub fn receive_word(&mut self, player: Id<UserMarker>, word: String) -> WordResult {
    if !words::is_word(word.clone()) {
      return WordResult::Invalid;
    }

    if self.use_count.contains_key(&word) {
      let count = self.use_count.get(&word).unwrap();
      if count >= &(self.num_players / 2) {
        return WordResult::Blocked;
      } else {
        self.use_count.insert(word.clone(), count+1);
      }
    } else {
      self.use_count.insert(word.clone(), 1);
    }

    for criteron in &self.criteria {
      if !criteron.test(word.clone()) {
        return WordResult::Invalid;
      }
    }

    self.scored_words.get_mut(&player).unwrap().push(word.clone());

    if words::deserves_bonus(word.clone()) {
      WordResult::ScoredBonus
    } else {
      WordResult::Scored
    }
  }

  pub fn end_round(&mut self) {
    for player in self.scored_words.keys() {
      let mut initial_score = *self.starting_scores.get(player).unwrap();
      for word in self.scored_words.get(player).unwrap() {
        initial_score += if words::deserves_bonus(word.clone()) { 2 } else { 1 };
      }
    }
  }
}

pub enum WordResult { Invalid, Scored, ScoredBonus, Blocked }

pub fn generate_rounds(players: &Vec<Id<UserMarker>>, num_rounds: u32) -> Vec<Round> {
  let mut rounds: Vec<Round> = vec![];

  for _ in 0..num_rounds {
    let mut zero_scores: HashMap<Id<UserMarker>, u32> = HashMap::new();

    for player in players {
      zero_scores.insert(*player, 0);
    }

    rounds.push(Round::new(zero_scores));
  }

  rounds
}
