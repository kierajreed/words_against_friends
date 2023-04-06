use twilight_model::{id::{marker::{ChannelMarker, UserMarker, MessageMarker}, Id}, channel::Message};

use crate::{round::{self, WordResult, Round}, discord::{CommonReactions, DiscordMinion}};

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
  rounds: Vec<Round>,
  round_index: i32,
  minion: DiscordMinion,
}

impl WordsAgainstStrangers {
  pub fn new(public_channel: Id<ChannelMarker>, wordsmith: Id<UserMarker>, minion: DiscordMinion) -> Self {
    Self {
      public_channel,
      state: GameState::Starting,
      players: vec![wordsmith],
      header_message: None,
      rounds: vec![],
      round_index: -1,
      minion,
    }
  }

  pub fn get_state(&self) -> GameState {
    self.state
  }

  pub fn advance_rounds(&mut self) {
    self.round_index += 1;

    if self.round_index == 0 {
      self.rounds = round::generate_rounds(&self.players, 3);
    }

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

  fn get_current_round(&mut self) -> &mut Round {
    self.rounds.get_mut(self.round_index as usize).unwrap()
  }

  pub fn make_intro(&self) -> String {
    String::from("**Words Against Friends**\nPlayers: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ")
  }

  pub fn get_starting_message(&self) -> String {
    String::from("**Words Against Friends**\nStarting now with players: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ") +
      "\n:warning: Go to your DMs to get ready to play!"
  }

  pub fn get_dm_opening(&self) -> String {
    String::from("**Words Against Friends**\nGet ready to play! Game starting soon...")
  }

  pub fn get_round_announcement(&self) -> String {
    format!("**Words Against Friends: Round {} of {}**\nSend me words that: ", self.round_index+1, self.rounds.len()) +
      &self.rounds.get(self.round_index as usize).unwrap().get_criteria_string()
  }

  pub async fn receive_word(&mut self, message: &Message, word: String) {
    let result = self.get_current_round().receive_word(message.author.id, word);
    let reaction = match result {
      WordResult::Invalid => CommonReactions::RedX,
      WordResult::Blocked => CommonReactions::OctagonalSign,
      WordResult::Scored => CommonReactions::CheckmarkGreen,
      WordResult::ScoredBonus => CommonReactions::CheckmarkBlue,
    };
    self.minion.add_reaction(message, reaction).await;
  }
}
