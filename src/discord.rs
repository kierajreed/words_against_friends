use std::collections::HashMap;
use twilight_model::{channel::Message, id::{marker::{ChannelMarker, GuildMarker, MessageMarker}, Id}};
use twilight_http::{Client as HttpClient, request::channel::reaction::RequestReactionType, Response, response::marker::EmptyBody};
use crate::game::{WordsAgainstStrangers, GameState};

pub struct DiscordBot {
  prefix: String,
  games: HashMap<Id<GuildMarker>, WordsAgainstStrangers>,
  dm_to_guild: HashMap<Id<ChannelMarker>, Id<GuildMarker>>,
  client: HttpClient,
}

impl DiscordBot {
  pub fn new(prefix: String, token: String) -> Self {
    Self {
      prefix,
      games: HashMap::new(),
      dm_to_guild: HashMap::new(),
      client: HttpClient::new(token),
    }
  }

  pub async fn handle_message(&mut self, message: Message) {
    if message.content.starts_with(&self.prefix) {
      if message.guild_id.is_none() {
        self.send_message(message.channel_id, CommonMessages::NoDmCommands.val()).await;
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
    } else if self.dm_to_guild.contains_key(&message.channel_id) {

    }
  }

  async fn new_game(&mut self, message: Message) {
    if self.games.contains_key(&message.guild_id.unwrap()) {
      self.send_message(message.channel_id, CommonMessages::ExistingGame.val()).await;
      return;
    }

    let mut new_game = WordsAgainstStrangers::new(message.channel_id, message.author.id);
    let intro = self.send_message(message.channel_id, new_game.make_intro()).await;
    new_game.set_header(intro.id);
    self.games.insert(message.guild_id.unwrap(), new_game);
  }
  async fn join_game(&mut self, message: Message) {
    if !self.games.contains_key(&message.guild_id.unwrap()) {
      self.send_message(message.channel_id, CommonMessages::NoExistingGame.val()).await;
      return;
    }
    if self.games.get(&message.guild_id.unwrap()).unwrap().get_state() != GameState::Starting {
      self.send_message(message.channel_id, CommonMessages::GameInProgress.val()).await;
      return;
    }

    self.games.get_mut(&message.guild_id.unwrap()).unwrap().add_player(message.author.id);

    let joining_game = self.games.get(&message.guild_id.unwrap()).unwrap();
    self.edit_message(joining_game.get_active_channel(), joining_game.get_header(), joining_game.make_intro()).await;
    self.add_reaction(&message, &CommonReactions::CheckmarkGreen.val()).await;
  }
  async fn start_game(&mut self, message: Message) {
    if !self.games.contains_key(&message.guild_id.unwrap()) {
      self.send_message(message.channel_id, CommonMessages::NoExistingGame.val()).await;
      return;
    }
    if self.games.get(&message.guild_id.unwrap()).unwrap().get_state() != GameState::Starting {
      self.send_message(message.channel_id, CommonMessages::GameInProgress.val()).await;
      return;
    }
    if *self.games.get(&message.guild_id.unwrap()).unwrap().get_players().get(0).unwrap() != message.author.id {
      self.send_message(message.channel_id, CommonMessages::NoPermission.val()).await;
      return;
    }

    self.games.get_mut(&message.guild_id.unwrap()).unwrap().advance_rounds();

    let starting_game = self.games.get(&message.guild_id.unwrap()).unwrap();
    self.send_message(message.channel_id, starting_game.get_starting_message()).await;

    self.dm_all_players(&message.guild_id.unwrap(), starting_game.get_dm_opening()).await;
  }


  async fn send_message(&self, channel: Id<ChannelMarker>, content: String) -> Message {
    self.client.create_message(channel).content(&content).unwrap().await.unwrap().model().await.unwrap()
  }

  async fn edit_message(&self, channel: Id<ChannelMarker>, message: Id<MessageMarker>, content: String) {
    self.client.update_message(channel, message).content(Some(&content)).unwrap().await.unwrap();
  }

  async fn add_reaction(&self, message: &Message, reaction: &RequestReactionType<'_>) -> Response<EmptyBody> {
    self.client.create_reaction(message.channel_id, message.id, reaction).await.unwrap()
  }

  async fn dm_all_players(&self, game_server: &Id<GuildMarker>, content: String) {
    let players = self.games.get(&game_server).unwrap().get_players();

    for player_id in players {
      let dm_channel = self.client.create_private_channel(*player_id).await.unwrap().model().await.unwrap();

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
}

impl CommonMessages {
  fn val(&self) -> String {
    let message = match *self {
      Self::NoDmCommands => "You cannot use commands in direct messages.",
      Self::ExistingGame => "There is already a game in this server!",
      Self::NoExistingGame => "There is no game in this server yet!",
      Self::GameInProgress => "This game is in progress, you cannot do that!",
      Self::NoPermission => "Only the player who created the game may start it!",
    };
    message.to_string()
  }
}

enum CommonReactions {
  CheckmarkGreen,
}

impl CommonReactions {
  fn val(&self) -> RequestReactionType {
    let emoji = match *self {
      Self::CheckmarkGreen => "✅"
    };
    RequestReactionType::Unicode { name: emoji }
  }
}
