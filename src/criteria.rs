use std::fmt;
use std::fmt::Display;

use crate::words;

pub trait Criteron {
  fn test(&self, word: String) -> bool;
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
}
impl Criteron for EndsWith {
  fn test(&self, word: String) -> bool {
    word.ends_with(&self.pattern)
  }
}
impl Criteron for Contains {
  fn test(&self, word: String) -> bool {
    word.contains(&self.pattern)
  }
}
impl Criteron for OfLength {
  fn test(&self, word: String) -> bool {
    word.len() == self.length
  }
}
impl Criteron for MinLength {
  fn test(&self, word: String) -> bool {
    word.len() >= self.length
  }
}
impl Criteron for RhymesWith {
  fn test(&self, word: String) -> bool {
    words::is_rhyme(word, self.word.clone())
  }
}
impl Criteron for PartOfSpeech {
  fn test(&self, word: String) -> bool {
    words::get_part_of_speech(word) == self.part_of_speech
  }
}

impl Display for StartsWith {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Starts with `{}`", self.pattern)
  }
}
impl Display for EndsWith {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Ends with `{}`", self.pattern)
  }
}
impl Display for Contains {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Contains `{}`", self.pattern)
  }
}
impl Display for OfLength {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Exactly `{}` letters long", self.length)
  }
}
impl Display for MinLength {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "At least `{}` letters long", self.length)
  }
}
impl Display for RhymesWith {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Rhymes with \"`{}`\"", self.word)
  }
}
impl Display for PartOfSpeech {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Is a `{}`", self.part_of_speech.to_string())
  }
}
