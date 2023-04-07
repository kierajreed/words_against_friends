use std::collections::HashMap;
use twilight_model::{channel::Message, id::{marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker}, Id}};
use twilight_http::{Client as HttpClient, request::channel::reaction::RequestReactionType, Response, response::marker::EmptyBody};
use crate::game::{WordsAgainstStrangers, GameState};

pub struct DiscordBot {
  prefix: String,
  games: HashMap<Id<GuildMarker>, WordsAgainstStrangers>,
  dm_to_guild: HashMap<Id<UserMarker>, Id<GuildMarker>>,
  minion: DiscordMinion,
  token: String,
}

impl DiscordBot {
  pub fn new(prefix: String, token: String) -> Self {
    Self {
      prefix,
      games: HashMap::new(),
      dm_to_guild: HashMap::new(),
      minion: DiscordMinion::new(token.clone()),
      token,
    }
  }

  pub async fn handle_message(&mut self, message: Message) {
    if self.dm_to_guild.contains_key(&message.author.id) && message.guild_id.is_none() {
      let guild_id = self.dm_to_guild.get(&message.author.id).unwrap();
      let relevant_game = self.games.get_mut(guild_id).unwrap();
      relevant_game.receive_word(&message, message.content.clone()).await;
    }

    if message.content.clone().starts_with(&self.prefix) {
      if message.guild_id.is_none() {
        self.minion.send_message(message.channel_id, CommonMessages::NoDmCommands.val()).await;
        return;
      }

      let prefix_removed = message.content.strip_prefix(&self.prefix).unwrap().to_string();
      let mut chunks: Vec<&str> = prefix_removed.split(" ").collect();
      let command = chunks.pop().unwrap();

      match command {
        "new" => { self.new_game(message).await; }
        "join" => { self.join_game(message).await; }
        "start" => { self.start_game(message).await; }
        _ => {}
      }
    }
  }

  async fn new_game(&mut self, message: Message) {
    if self.games.contains_key(&message.guild_id.unwrap()) {
      self.minion.send_message(message.channel_id, CommonMessages::ExistingGame.val()).await;
      return;
    }
    if self.dm_to_guild.contains_key(&message.author.id) {
      self.minion.send_message(message.channel_id, CommonMessages::AlreadyInGame.val()).await;
      return;
    }

    let new_game = WordsAgainstStrangers::new(
      message.channel_id,
      message.author.id,
      DiscordMinion::new(self.token.clone())
    ).await;
    self.games.insert(message.guild_id.unwrap(), new_game);
  }
  async fn join_game(&mut self, message: Message) {
    if !self.games.contains_key(&message.guild_id.unwrap()) {
      self.minion.send_message(message.channel_id, CommonMessages::NoExistingGame.val()).await;
      return;
    }
    if self.games.get(&message.guild_id.unwrap()).unwrap().get_state() != GameState::Starting {
      self.minion.send_message(message.channel_id, CommonMessages::GameInProgress.val()).await;
      return;
    }
    if self.dm_to_guild.contains_key(&message.author.id) {
      self.minion.send_message(message.channel_id, CommonMessages::AlreadyInGame.val()).await;
      return;
    }

    self.games.get_mut(&message.guild_id.unwrap()).unwrap().add_player(message.author.id).await;
    self.minion.add_reaction(&message, CommonReactions::CheckmarkGreen).await;
  }
  async fn start_game(&mut self, message: Message) {
    if !self.games.contains_key(&message.guild_id.unwrap()) {
      self.minion.send_message(message.channel_id, CommonMessages::NoExistingGame.val()).await;
      return;
    }
    if self.games.get(&message.guild_id.unwrap()).unwrap().get_state() != GameState::Starting {
      self.minion.send_message(message.channel_id, CommonMessages::GameInProgress.val()).await;
      return;
    }
    if self.games.get(&message.guild_id.unwrap()).unwrap().get_players().get(0).unwrap() != &message.author.id {
      self.minion.send_message(message.channel_id, CommonMessages::NoPermission.val()).await;
      return;
    }

    for player in self.games.get(&message.guild_id.unwrap()).unwrap().get_players() {
      self.dm_to_guild.insert(*player, message.guild_id.unwrap());
    }

    self.games.get_mut(&message.guild_id.unwrap()).unwrap().start().await;
  }
}

pub struct DiscordMinion {
  client: HttpClient,
}

impl DiscordMinion {
  fn new(token: String) -> Self {
    Self {
      client: HttpClient::new(token),
    }
  }

  pub async fn send_message(&self, channel: Id<ChannelMarker>, content: String) -> Message {
    self.client.create_message(channel).content(&content).unwrap().await.unwrap().model().await.unwrap()
  }

  pub async fn edit_message(&self, channel: Id<ChannelMarker>, message: Id<MessageMarker>, new_content: String) {
    self.client.update_message(channel, message).content(Some(&new_content)).unwrap().await.unwrap();
  }

  pub async fn add_reaction(&self, message: &Message, reaction: CommonReactions) -> Response<EmptyBody> {
    self.client.create_reaction(message.channel_id, message.id, &reaction.val()).await.unwrap()
  }

  pub async fn dm_all(&self, users: &Vec<Id<UserMarker>>, content: String) {
    for user_id in users {
      let dm_channel = self.client.create_private_channel(*user_id).await.unwrap().model().await.unwrap();

      self.send_message(dm_channel.id, content.clone()).await;
    }
  }
}

enum CommonMessages {
  NoDmCommands,
  ExistingGame,
  NoExistingGame,
  GameInProgress,
  NoPermission,
  AlreadyInGame,
}

impl CommonMessages {
  fn val(&self) -> String {
    let message = match *self {
      Self::NoDmCommands => "You cannot use commands in direct messages.",
      Self::ExistingGame => "There is already a game in this server!",
      Self::NoExistingGame => "There is no game in this server yet!",
      Self::GameInProgress => "This game is in progress, you cannot do that!",
      Self::NoPermission => "Only the player who created the game may start it!",
      Self::AlreadyInGame => "You may only join a game in one server at a time!",
    };
    message.to_string()
  }
}

pub enum CommonReactions {
  CheckmarkGreen,
  CheckmarkBlue,
  OctagonalSign,
  RedX,
}

impl CommonReactions {
  fn val(&self) -> RequestReactionType {
    let emoji = match *self {
      Self::CheckmarkGreen => "✅",
      Self::CheckmarkBlue => "☑️",
      Self::OctagonalSign => "🛑",
      Self::RedX => "❌",
    };
    RequestReactionType::Unicode { name: emoji }
  }
}
