use tokio::time::{sleep, Duration};
use twilight_model::{id::{marker::{ChannelMarker, UserMarker, MessageMarker}, Id}, channel::Message};

use crate::{round::{self, WordResult, Round}, discord::{CommonReactions, DiscordMinion}};

const NUM_ROUNDS: u32 = 3;
const ROUND_DURATION: u64 = 15;

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
  pub async fn new(public_channel: Id<ChannelMarker>, wordsmith: Id<UserMarker>, minion: DiscordMinion) -> Self {
    let mut new_game = Self {
      public_channel,
      state: GameState::Starting,
      players: vec![wordsmith],
      header_message: None,
      rounds: vec![],
      round_index: 0,
      minion,
    };
    let intro = new_game.minion.send_message(public_channel, new_game.make_intro()).await;
    new_game.header_message = Some(intro.id);

    new_game
  }

  pub async fn start(&mut self) {
    self.state = GameState::BetweenRounds;
    self.rounds = round::generate_rounds(&self.players, NUM_ROUNDS);
    self.minion.send_message(self.public_channel, self.get_starting_message()).await;
    self.minion.dm_all(self.get_players(), self.get_dm_opening()).await;
    sleep(Duration::from_millis(3000)).await;

    for _ in 0..self.rounds.len() {
      self.state = GameState::ActivePlay;
      self.minion.dm_all(self.get_players(), self.get_round_announcement()).await;

      sleep(Duration::from_secs(ROUND_DURATION)).await;
      self.state = GameState::BetweenRounds;
      self.round_index += 1;
      // announce scores
      // do it all again
    }

    // announce game end
    // announce scores
  }

  pub async fn add_player(&mut self, player: Id<UserMarker>) {
    self.players.push(player);

    self.minion.edit_message(self.public_channel, self.header_message.unwrap(), self.make_intro()).await;
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

  pub fn get_state(&self) -> GameState {
    self.state
  }

  pub fn get_players(&self) -> &Vec<Id<UserMarker>> {
    &self.players
  }

  fn get_current_round(&mut self) -> &mut Round {
    self.rounds.get_mut(self.round_index as usize).unwrap()
  }

  fn make_intro(&self) -> String {
    String::from("**Words Against Friends**\nPlayers: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ")
  }

  fn get_starting_message(&self) -> String {
    String::from("**Words Against Friends**\nStarting now with players: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ") +
      "\n:warning: Go to your DMs to get ready to play!"
  }

  fn get_dm_opening(&self) -> String {
    String::from("**Words Against Friends**\nGet ready to play! Game starting soon...")
  }

  fn get_round_announcement(&self) -> String {
    format!("**Words Against Friends: Round {} of {}**\nSend me words that: ", self.round_index+1, self.rounds.len()) +
      &self.rounds.get(self.round_index as usize).unwrap().get_criteria_string()
  }
}
