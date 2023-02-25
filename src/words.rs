use std::process::Command;

use ttaw;

#[derive(PartialEq)]
pub enum PartsOfSpeech { Noun, Verb, Adverb, Adjective }

impl PartsOfSpeech {
  pub fn to_string(&self) -> String {
    String::from(match self {
      Self::Noun => "noun",
      Self::Verb => "verb",
      Self::Adverb => "adverb",
      Self::Adjective => "adjective",
    })
  }

  pub fn wordpos_opt(&self) -> &str {
    match self {
      Self::Noun => "-n",
      Self::Verb => "-v",
      Self::Adverb => "-r",
      Self::Adjective => "-a",
    }
  }
}

pub fn is_word(word: String) -> bool {
  true // TODO
}

pub fn deserves_bonus(word: String) -> bool {
  false // TODO
}

pub fn is_rhyme(first: String, second: String) -> bool {
  ttaw::metaphone::rhyme(&first, &second)
}

pub fn is_part_of_speech(word: String, pos: &PartsOfSpeech) -> bool {
  let command = Command::new("cmd").args(&["/C", &format!("wordpos get -c {} {}", pos.wordpos_opt(), &word)]).output().unwrap();
  let output = String::from_utf8(command.stdout).unwrap();
  let counts = output.lines().nth(1).unwrap().split_whitespace();
  let mut total = 0;
  for count in counts {
    if count.contains("1") {
      total += 1;
    }
  }

  total == 2
}
