use twilight_model::id::{marker::{ChannelMarker, UserMarker, MessageMarker}, Id};

use crate::round;

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
  Starting,
  BetweenRounds,
  ActivePlay,
}

pub struct WordsAgainstStrangers {
  public_channel: Id<ChannelMarker>,
  state: GameState,
  players: Vec<Id<UserMarker>>,
  header_message: Option<Id<MessageMarker>>,
  rounds: Vec<round::Round>,
  round_index: i32,
}

impl WordsAgainstStrangers {
  pub fn new(public_channel: Id<ChannelMarker>, wordsmith: Id<UserMarker>) -> Self {
    Self {
      public_channel,
      state: GameState::Starting,
      players: vec![wordsmith],
      header_message: None,
      rounds: round::generate_rounds(3),
      round_index: -1
    }
  }

  pub fn get_state(&self) -> GameState {
    self.state
  }

  pub fn advance_rounds(&mut self) {
    self.round_index += 1;
    self.state = GameState::BetweenRounds;
  }
  pub fn start_active_play(&mut self) {
    self.state = GameState::ActivePlay;
  }

  pub fn get_active_channel(&self) -> Id<ChannelMarker> {
    self.public_channel
  }

  pub fn set_header(&mut self, header: Id<MessageMarker>) {
    self.header_message = Some(header);
  }
  pub fn get_header(&self) -> Id<MessageMarker> {
    self.header_message.unwrap()
  }

  pub fn get_players(&self) -> &Vec<Id<UserMarker>> {
    &self.players
  }
  pub fn add_player(&mut self, player: Id<UserMarker>) {
    self.players.push(player);
  }

  fn get_current_round(&mut self) -> &mut round::Round {
    self.rounds.get_mut(self.round_index as usize).unwrap()
  }

  pub fn make_intro(&self) -> String {
    String::from("**Words Against Friends**\nPlayers: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ")
  }

  pub fn get_starting_message(&self) -> String {
    String::from("**Words Against Friends**\nStarting now with players: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ") +
      ":warning: Go to your DMs to get ready to play!"
  }

  pub fn get_dm_opening(&self) -> String {
    String::from("**Words Against Friends**\nGet ready to play! Game starting soon...")
  }

  pub async fn receive_word(&mut self, player: Id<UserMarker>, word: String) -> round::WordResult {
    self.get_current_round().receive_word(player, word).await
  }
}
