use twilight_model::id::{marker::{ChannelMarker, UserMarker, MessageMarker}, Id};

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
  header_message: Option<Id<MessageMarker>>
}

impl WordsAgainstStrangers {
  pub fn new(public_channel: Id<ChannelMarker>, wordsmith: Id<UserMarker>) -> Self {
    Self {
      public_channel,
      state: GameState::Starting,
      players: vec![wordsmith],
      header_message: None
    }
  }

  pub fn get_state(&self) -> GameState {
    self.state
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

  pub fn add_player(&mut self, player: Id<UserMarker>) {
    self.players.push(player);
  }

  pub fn make_intro(&self) -> String {
    String::from("**Words Against Friends**\nPlayers: ") +
      &self.players.iter().map(|x| format!("<@!{}>", x)).collect::<Vec<_>>().join(", ")
  }
}
