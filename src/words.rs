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
}

pub fn is_word(word: String) -> bool {
  true // TODO
}

pub fn deserves_bonus(word: String) -> bool {
  false // TODO
}

pub fn is_rhyme(first: String, second: String) -> bool {
  true // TODO
}

pub fn get_part_of_speech(word: String) -> PartsOfSpeech {
  PartsOfSpeech::Noun // TODO
}
