use crate::words;
use rand::{thread_rng, distributions::WeightedIndex, prelude::Distribution};

pub struct Criteron {
  pattern: Option<String>,
  length: Option<usize>,
  part_of_speech: Option<words::PartsOfSpeech>,
  criteron_type: CriteronType,
}

#[derive(PartialEq)]
enum CriteronType {
  StartsWith,
  EndsWith,
  Contains,
  OfLength,
  MinLength,
  RhymesWith,
  PartOfSpeech,
}

impl Criteron {
  pub fn test(&self, word: String) -> bool {
    match &self.criteron_type {
      CriteronType::StartsWith => word.starts_with(&self.pattern.clone().unwrap()),
      CriteronType::EndsWith => word.ends_with(&self.pattern.clone().unwrap()),
      CriteronType::Contains => word.contains(&self.pattern.clone().unwrap()),
      CriteronType::OfLength => word.len() == self.length.unwrap(),
      CriteronType::MinLength => word.len() >= self.length.unwrap(),
      CriteronType::RhymesWith => words::is_rhyme(word, self.pattern.clone().unwrap()),
      CriteronType::PartOfSpeech => words::is_part_of_speech(word, &self.part_of_speech.clone().unwrap()),
    }
  }
  pub fn to_string(&self) -> String {
    match &self.criteron_type {
      CriteronType::StartsWith => format!("Start with `{}`", &self.pattern.clone().unwrap()),
      CriteronType::EndsWith => format!("End with `{}`", &self.pattern.clone().unwrap()),
      CriteronType::Contains => format!("Contain `{}`", &self.pattern.clone().unwrap()),
      CriteronType::OfLength => format!("Are exactly `{}` letters long", self.length.unwrap()),
      CriteronType::MinLength => format!("Are at least `{}` letters long", self.length.unwrap()),
      CriteronType::RhymesWith => format!("Rhyme with \"`{}`\"", &self.pattern.clone().unwrap()),
      CriteronType::PartOfSpeech => format!("Are `{}`s", self.part_of_speech.clone().unwrap().to_string())
    }
  }
}

mod make_criteria {
  use super::*;
  use rand::{Rng, thread_rng};

  pub fn starts_with() -> Criteron { // TODO: don't hard code lol
    Criteron {
      pattern: Some("te".to_owned()),
      length: None,
      part_of_speech: None,
      criteron_type: CriteronType::StartsWith,
    }
  }
  pub fn ends_with() -> Criteron { // TODO: don't hard code lol
    Criteron {
      pattern: Some("ge".to_owned()),
      length: None,
      part_of_speech: None,
      criteron_type: CriteronType::EndsWith,
    }
  }
  pub fn contains() -> Criteron { // TODO: don't hard code lol
    Criteron {
      pattern: Some("on".to_owned()),
      length: None,
      part_of_speech: None,
      criteron_type: CriteronType::Contains,
    }
  }
  pub fn of_length() -> Criteron {
    let mut rng = thread_rng();
    let length: usize = rng.gen_range(6..=10);
    Criteron {
      pattern: None,
      length: Some(length),
      part_of_speech: None,
      criteron_type: CriteronType::OfLength,
    }
  }
  pub fn min_length() -> Criteron {
    let mut rng = thread_rng();
    let length: usize = rng.gen_range(8..=11);
      Criteron {
        pattern: None,
        length: Some(length),
        part_of_speech: None,
        criteron_type: CriteronType::MinLength,
      }
  }
  pub fn rhymes_with() -> Criteron { // TODO: don't hard code lol
    Criteron {
      pattern: Some("finally".to_owned()),
      length: None,
      part_of_speech: None,
      criteron_type: CriteronType::RhymesWith,
    }
  }
  pub fn part_of_speech() -> Criteron { // TODO: don't hard code lol
    Criteron {
      pattern: None,
      length: None,
      part_of_speech: Some(words::PartsOfSpeech::Noun),
      criteron_type: CriteronType::PartOfSpeech,
    }
  }
}

fn generate_random_criteron(primary: bool) -> Criteron {
  let mut rng = thread_rng();

  if primary {
    let criteria_dist = WeightedIndex::new([50, 40, 20]).unwrap();

    match criteria_dist.sample(&mut rng) {
      0 => make_criteria::starts_with(),
      1 => make_criteria::ends_with(),
      2 => make_criteria::contains(),
      _ => panic!()
    }
  } else {
    let criteria_dist = WeightedIndex::new([20, 30, 10, 10]).unwrap();
    let selected = criteria_dist.sample(&mut rng);

    match selected {
      0 => make_criteria::of_length(),
      1 => make_criteria::min_length(),
      2 => make_criteria::rhymes_with(),
      3 => make_criteria::part_of_speech(),
      _ => panic!()
    }
  }
}

pub fn generate_random_criteria() -> Vec<Criteron> {
  let mut rng = thread_rng();

  let num_criteria_dist = WeightedIndex::new([100, 50]).unwrap();
  let num_criteria = num_criteria_dist.sample(&mut rng);

  let first = generate_random_criteron(true);
  let second = generate_random_criteron(false);
  if first.criteron_type == CriteronType::EndsWith && second.criteron_type == CriteronType::RhymesWith {
    return vec![second];
  }

  match num_criteria {
    0 => vec![first],
    1 => vec![first, second],
    _ => panic!()
  }
}
