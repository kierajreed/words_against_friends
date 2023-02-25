use crate::words;

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

pub fn generate_random_criteria() -> Vec<Box<dyn Criteron>> {
  vec![Box::new(RhymesWith { word: "batter".to_owned() })]
  // Box::new(PartOfSpeech { part_of_speech: words::PartsOfSpeech::Noun })
  // Box::new(StartsWith { pattern: "te".to_string() })] // TODO
}
