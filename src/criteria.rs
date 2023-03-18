use crate::words;
use rand::{Rng, thread_rng, distributions::WeightedIndex, prelude::Distribution};

pub trait Criteron {
  fn test(&self, word: String) -> bool;
  fn to_string(&self) -> String;
}

pub struct StartsWith { pattern: String }
pub struct EndsWith { pattern: String }
pub struct Contains { pattern: String }
pub struct OfLength { length: usize }
pub struct MinLength { length: usize }
pub struct RhymesWith { word: String }
pub struct PartOfSpeech { part_of_speech: words::PartsOfSpeech }

impl Criteron for StartsWith {
  fn test(&self, word: String) -> bool {
    word.starts_with(&self.pattern)
  }
  fn to_string(&self) -> String {
      format!("Starts with `{}`", self.pattern)
  }
}
impl Criteron for EndsWith {
  fn test(&self, word: String) -> bool {
    word.ends_with(&self.pattern)
  }
  fn to_string(&self) -> String {
    format!("Ends with `{}`", self.pattern)
  }
}
impl Criteron for Contains {
  fn test(&self, word: String) -> bool {
    word.contains(&self.pattern)
  }
  fn to_string(&self) -> String {
    format!("Contains `{}`", self.pattern)
  }
}
impl Criteron for OfLength {
  fn test(&self, word: String) -> bool {
    word.len() == self.length
  }
  fn to_string(&self) -> String {
    format!("Exactly `{}` letters long", self.length)
  }
}
impl Criteron for MinLength {
  fn test(&self, word: String) -> bool {
    word.len() >= self.length
  }
  fn to_string(&self) -> String {
    format!("At least `{}` letters long", self.length)
  }
}
impl Criteron for RhymesWith {
  fn test(&self, word: String) -> bool {
    words::is_rhyme(word, self.word.clone())
  }
  fn to_string(&self) -> String {
    format!("Rhymes with \"`{}`\"", self.word)
  }
}
impl Criteron for PartOfSpeech {
  fn test(&self, word: String) -> bool {
    words::is_part_of_speech(word, &self.part_of_speech)
  }
  fn to_string(&self) -> String {
    format!("Is a `{}`", self.part_of_speech.to_string())
  }
}

mod make_criteria {
  use super::*;
  use rand::{Rng, thread_rng};

  pub fn starts_with() -> Box<StartsWith> { // TODO: don't hard code lol
    Box::new(StartsWith { pattern: "te".to_owned() })
  }
  pub fn ends_with() -> Box<EndsWith> { // TODO: don't hard code lol
    Box::new(EndsWith { pattern: "ge".to_owned() })
  }
  pub fn contains() -> Box<Contains> { // TODO: don't hard code lol
    Box::new(Contains { pattern: "on".to_owned() })
  }
  pub fn of_length() -> Box<OfLength> {
    let mut rng = thread_rng();
    let length: usize = rng.gen_range(6..=10);
    Box::new(OfLength { length })
  }
  pub fn min_length() -> Box<MinLength> {
    let mut rng = thread_rng();
    let length: usize = rng.gen_range(8..=11);
    Box::new(MinLength { length })
  }
  pub fn rhymes_with() -> Box<RhymesWith> { // TODO: don't hard code lol
    Box::new(RhymesWith { word: "finally".to_owned() })
  }
  pub fn part_of_speech() -> Box<PartOfSpeech> { // TODO: don't hard code lol
    Box::new(PartOfSpeech { part_of_speech: words::PartsOfSpeech::Noun })
  }
}

fn generate_random_criteron(exclude: &mut Vec<usize>) -> Box<dyn Criteron> {
  let mut rng = thread_rng();

  if exclude.len() == 0 {
    let criteria_dist = WeightedIndex::new([50, 40, 20]).unwrap();

    match criteria_dist.sample(&mut rng) {
      0 => make_criteria::starts_with(),
      1 => make_criteria::ends_with(),
      2 => make_criteria::contains(),
      _ => panic!()
    }
  } else {
    let criteria_dist = WeightedIndex::new([20, 30, 10, 10]).unwrap();
    let mut selected = criteria_dist.sample(&mut rng);

    while exclude.contains(&selected) {
      selected = criteria_dist.sample(&mut rng);
    }
    exclude.push(selected);

    match selected {
      0 => make_criteria::of_length(),
      1 => make_criteria::min_length(),
      2 => make_criteria::rhymes_with(),
      3 => make_criteria::part_of_speech(),
      _ => panic!()
    }
  }
}

pub fn generate_random_criteria() -> Vec<Box<dyn Criteron>> {
  let mut criteria: Vec<Box<dyn Criteron>> = vec![];

  let mut rng = thread_rng();

  let num_criteria_dist = WeightedIndex::new([100, 50]).unwrap();
  let num_criteria = num_criteria_dist.sample(&mut rng);
  let mut excluded: Vec<usize> = vec![];

  for _ in 0..num_criteria {
    criteria.push(generate_random_criteron(&mut excluded));
  }

  criteria
}
