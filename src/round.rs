use std::collections::HashMap;

use twilight_model::id::{marker::UserMarker, Id};

use crate::criteria;
use crate::words;

pub struct Round {
  starting_scores: HashMap<Id<UserMarker>, u32>,
  updated_scores: HashMap<Id<UserMarker>, u32>,
  scored_words: HashMap<Id<UserMarker>, Vec<String>>,
  use_count: HashMap<String, u32>,
  criteria: Vec<Box<dyn criteria::Criteron>>,
  num_players: u32
}

impl Round {
  pub async fn receive_word(&mut self, player: Id<UserMarker>, word: String) -> WordResult {
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

    self.scored_words.get_mut(&player).unwrap().push(word.clone());

    if words::deserves_bonus(word.clone()) {
      WordResult::ScoredBonus
    } else {
      WordResult::Scored
    }
  }
}

pub enum WordResult { Invalid, Scored, ScoredBonus, Blocked }

pub fn generate_rounds(num_rounds: u32) -> Vec<Round> {
  vec![] // TODO
}
